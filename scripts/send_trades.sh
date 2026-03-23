#!/usr/bin/env bash

set -euo pipefail

URL="http://localhost:8656/trade"

send_trade() {
  local msg="$1"

  echo "--------------------"

  local response
  local body
  local status

  response=$(
    curl -sS -X POST "$URL" \
      -H "Content-Type: application/json" \
      -d "$(jq -n --arg text "$msg" '{text: $text}')" \
      -w $'\n%{http_code}'
  )

  body="$(echo "$response" | sed '$d')"
  status="$(echo "$response" | tail -n1)"

  echo "HTTP Status: $status"
  echo "Response Body:"

  if command -v jq >/dev/null 2>&1; then
    echo "$body" | jq . 2>/dev/null || echo "$body"
  else
    echo "$body"
  fi

}

# ===================== MESSAGES =====================

# --- SHORTS (entry at current price, TPs below, SL above) ---

send_trade "🔔 BTCUSDT · 1h · 🔴 SHORT
🎯 Entry: 69900
🎯 Targets:
   🥇 TP1: 69200  ⬇️ 1.00%  ⚖️ R:R 0.50
   🥈 TP2: 68500  ⬇️ 2.00%
   🥉 TP3: 67800  ⬇️ 3.00%
🛡️ SL: 71300  ❗️Risk 2.00%"

send_trade "🔔 XRPUSDT · 30m · 🔴 SHORT
🎯 Entry: 1.4100
🎯 Targets:
   🥇 TP1: 1.3900  ⬇️ 1.42%  ⚖️ R:R 0.50
   🥈 TP2: 1.3700  ⬇️ 2.84%
   🥉 TP3: 1.3500  ⬇️ 4.26%
🛡️ SL: 1.4500  ❗️Risk 2.84%"

send_trade "🔔 SOLUSDT · 30m · 🔴 SHORT
🎯 Entry: 89.00
🎯 Targets:
   🥇 TP1: 88.00  ⬇️ 1.12%  ⚖️ R:R 0.50
   🥈 TP2: 87.00  ⬇️ 2.25%
   🥉 TP3: 86.00  ⬇️ 3.37%
🛡️ SL: 91.00  ❗️Risk 2.25%"

# --- LONGS (entry at current price, TPs above, SL below) ---

send_trade "🔔 ETHUSDT · 30m · 🟢 LONG
🎯 Entry: 2120
🎯 Targets:
   🥇 TP1: 2150  ⬆️ 1.42%  ⚖️ R:R 0.50
   🥈 TP2: 2180  ⬆️ 2.83%
   🥉 TP3: 2210  ⬆️ 4.25%
🛡️ SL: 2060  ❗️Risk 2.83%"

send_trade "🔔 ADAUSDT · 30m · 🟢 LONG
🎯 Entry: 0.2580
🎯 Targets:
   🥇 TP1: 0.2620  ⬆️ 1.55%  ⚖️ R:R 0.50
   🥈 TP2: 0.2660  ⬆️ 3.10%
   🥉 TP3: 0.2700  ⬆️ 4.65%
🛡️ SL: 0.2500  ❗️Risk 3.10%"

send_trade "🔔 BNBUSDT · 1h · 🟢 LONG
🎯 Entry: 639
🎯 Targets:
   🥇 TP1: 648  ⬆️ 1.41%  ⚖️ R:R 0.50
   🥈 TP2: 657  ⬆️ 2.82%
   🥉 TP3: 666  ⬆️ 4.23%
🛡️ SL: 621  ❗️Risk 2.82%"

echo "Done."