pub mod engine;
pub mod voxels;

#[tokio::main]
async fn main() {
    engine::run().await;
}
