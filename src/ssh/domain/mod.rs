pub mod group;
pub mod server;
pub mod credentials;


use async_trait::async_trait;

use crate::error::Result;


#[async_trait]
pub trait SshStore {
  async fn initialize(&self) -> Result<()>;
}