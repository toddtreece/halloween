use async_trait::async_trait;

#[async_trait]
pub trait Control {
  async fn run(&mut self);
}
