mod bootstrap;
mod execution_policy;
mod pipeline;
mod runtime;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    let runtime_deps = bootstrap::bootstrap();
    runtime::run_runtime(runtime_deps).await;
}
