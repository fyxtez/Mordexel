use crate::bootstrap::run;

mod bootstrap;
mod execution_policy;
mod pipeline;
mod utils;

#[tokio::main]
async fn main() {
    run().await;
}
