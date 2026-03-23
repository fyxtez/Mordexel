#!/usr/bin/env bash
set -euo pipefail
trap 'echo ""; echo "❌ Deployment failed at line $LINENO"; exit 1' ERR

# --------------------------
# SAFETY CHECKS
# --------------------------

if ! command -v git >/dev/null 2>&1; then
    echo "git is not installed or not available in PATH"
    exit 1
fi

if [ -n "$(git status --porcelain)" ]; then
    echo "Working tree is dirty. Commit changes before deploy."
    exit 1
fi

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [ "$BRANCH" != "master" ]; then
    echo "Not on master branch. Current branch: $BRANCH"
    exit 1
fi

# --------------------------
# LOAD CONFIG
# --------------------------

if [ ! -f .env.deploy ]; then
    echo "Error: .env.deploy file not found!"
    exit 1
fi

set -a
source .env.deploy
set +a

: "${REMOTE_USER:?Missing REMOTE_USER}"
: "${REMOTE_HOST:?Missing REMOTE_HOST}"
: "${REMOTE_APP_DIR:?Missing REMOTE_APP_DIR}"
: "${REMOTE_BIN_PATH:?Missing REMOTE_BIN_PATH}"
: "${REMOTE_ENV_PATH:?Missing REMOTE_ENV_PATH}"
: "${REMOTE_SESSION_PATH:?Missing REMOTE_SESSION_PATH}"
: "${SERVICE_NAME:?Missing SERVICE_NAME}"
: "${LOCAL_WORKSPACE_DIR:?Missing LOCAL_WORKSPACE_DIR}"
: "${LOCAL_BINARY_PATH:?Missing LOCAL_BINARY_PATH}"
: "${REMOTE_WORKING_DIR:?Missing REMOTE_WORKING_DIR}"

echo "Deploying Mordexel to $REMOTE_USER@$REMOTE_HOST"
read -r -p "Continue? (y/N): " CONFIRM
if [[ "$CONFIRM" != "y" ]]; then
    echo "Aborted."
    exit 0
fi

# --------------------------
# BUILD
# --------------------------

echo "Moving to workspace directory..."
cd "$LOCAL_WORKSPACE_DIR"

BUILD_VERSION="$(git rev-parse --short HEAD)"
if [ -z "$BUILD_VERSION" ]; then
    echo "Failed to determine git hash for BUILD_VERSION"
    exit 1
fi

echo "Build version: $BUILD_VERSION"
echo "Building project in release mode..."
BUILD_VERSION="$BUILD_VERSION" cargo build --release

if [ ! -x "$LOCAL_BINARY_PATH" ]; then
    echo "Binary not found or not executable at $LOCAL_BINARY_PATH"
    exit 1
fi

echo "Build succeeded."

# --------------------------
# OPTIONAL BINARY SIZE INFO
# --------------------------

HUMAN_SIZE="$(du -h "$LOCAL_BINARY_PATH" | cut -f1)"
BYTES_SIZE="$(stat -c%s "$LOCAL_BINARY_PATH")"
MB_SIZE="$(awk "BEGIN {printf \"%.2f\", $BYTES_SIZE/1024/1024}")"

echo "Binary size:"
echo "   • Human readable: $HUMAN_SIZE"
echo "   • Exact bytes:    $BYTES_SIZE bytes"
echo "   • In MB:          $MB_SIZE MB"

DEPLOY_START_TIME="$(date +"%Y-%m-%d %H:%M:%S")"

# --------------------------
# PREPARE REMOTE DIRECTORIES
# --------------------------

echo "Preparing remote directories..."
ssh "$REMOTE_USER@$REMOTE_HOST" "
    mkdir -p '$REMOTE_APP_DIR/bin'
"

# --------------------------
# COPY FILES
# --------------------------

echo "Stopping existing service..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl stop '$SERVICE_NAME' 2>/dev/null || true"

echo "Copying binary..."
scp "$LOCAL_BINARY_PATH" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_BIN_PATH"

echo "Setting permissions..."
ssh "$REMOTE_USER@$REMOTE_HOST" "
    chmod +x '$REMOTE_BIN_PATH'
    chmod 600 '$REMOTE_SESSION_PATH' || true
"

# --------------------------
# ENSURE SYSTEMD SERVICE
# --------------------------

echo "Ensuring systemd service exists..."

ssh "$REMOTE_USER@$REMOTE_HOST" "
if [ ! -f /etc/systemd/system/$SERVICE_NAME.service ]; then
    cat > /etc/systemd/system/$SERVICE_NAME.service << EOF
[Unit]
Description=Mordexel Engine
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=$REMOTE_WORKING_DIR
EnvironmentFile=-$REMOTE_ENV_PATH
Environment=RUST_LOG=info
Environment=SESSION_FILE=$REMOTE_SESSION_PATH
ExecStart=$REMOTE_BIN_PATH
Restart=always
RestartSec=5

StandardOutput=journal
StandardError=journal

LimitNOFILE=65536

# Optional light protection against runaway memory
# MemoryMax=700M

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl enable $SERVICE_NAME
    echo 'Service created and enabled.'
else
    echo 'Service already exists. Skipping creation.'
fi
"

# --------------------------
# RESTART
# --------------------------

echo "Restarting service..."
ssh "$REMOTE_USER@$REMOTE_HOST" "
    systemctl daemon-reload
    systemctl restart '$SERVICE_NAME'
"

echo "Waiting for service to become active..."
sleep 2

if ! ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl is-active --quiet '$SERVICE_NAME'"; then
    echo "Service is not active!"
    ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl status '$SERVICE_NAME' --no-pager"
    echo ""
    echo "Recent logs:"
    ssh "$REMOTE_USER@$REMOTE_HOST" "journalctl -u '$SERVICE_NAME' -n 50 --no-pager"
    exit 1
fi

echo "Service is active."

# --------------------------
# VERIFY BUILD VERSION IN JOURNAL
# --------------------------

echo "Checking logs for build version..."
LOG_OUTPUT="$(ssh "$REMOTE_USER@$REMOTE_HOST" "journalctl -u '$SERVICE_NAME' -n 100 --no-pager")"

if echo "$LOG_OUTPUT" | grep -Fq "$BUILD_VERSION"; then
    echo "Build version $BUILD_VERSION found in logs ✅"
else
    echo "Warning: build version $BUILD_VERSION not found in recent logs."
    echo "Make sure your app logs BUILD_VERSION on startup."
fi

# --------------------------
# FINISH
# --------------------------

DEPLOY_END_TIME="$(date +"%Y-%m-%d %H:%M:%S")"

echo ""
echo "✅ Deployment complete!"
echo "Started at : $DEPLOY_START_TIME"
echo "Finished at: $DEPLOY_END_TIME"
echo ""
echo "Useful commands:"
echo "   ssh $REMOTE_USER@$REMOTE_HOST 'systemctl status $SERVICE_NAME'"
echo "   ssh $REMOTE_USER@$REMOTE_HOST 'journalctl -u $SERVICE_NAME -f'"
echo "   ssh $REMOTE_USER@$REMOTE_HOST 'systemctl restart $SERVICE_NAME'"
echo "   ssh $REMOTE_USER@$REMOTE_HOST 'systemctl stop $SERVICE_NAME'"