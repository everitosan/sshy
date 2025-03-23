use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;

use super::super::dtos;
use crate::error::Result;


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Server {
  pub id: Uuid,
  pub group_id: Uuid,
  pub name: String,
  pub hostname: String,
  pub port: u32
}


#[async_trait]
pub trait SshyServerRepo {
  async fn create_server(&self, dto: dtos::CreateServerDto) -> Result<Server>;
  async fn list_servers(&self, group_id: Uuid) -> Result<Vec<Server>>;
  async fn update_server(&self, id: Uuid, dto: dtos::UpdateServerDto) -> Result<Server>;
}