use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

use crate::error::Result;
use super::dtos;


#[derive(Default)]
pub struct KeyPair {
  pub id: Uuid,
  pub server_id: Uuid,
  pub public: String,
  pub private: String
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Server {
  pub id: Uuid,
  pub group_id: Uuid,
  pub name: String,
  pub hostname: String,
  pub port: u32,
  pub user: String
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub name:  String,
  pub group: Option<Vec<Group>>,
  pub servers: Vec<Server>
}

#[async_trait]
pub trait SshStore {
  async fn initialize(&self) -> Result<()>;
  async fn create_group(&self, dto: dtos::CreateGroupDto) -> Result<Group>;
  async fn list_groups(&self, id: &Option<Uuid>) -> Result<Vec<Group>>;
  async fn get_group_by_id(&self, id: Uuid) -> Result<Option<Group>>;
  async fn update_group(&self, id: Uuid, dto: dtos::UpdateGroupDto) -> Result<Group>;
  async fn create_server(&self, dto: dtos::CreateServerDto) -> Result<Server>;
  async fn list_servers(&self, group_id: Uuid) -> Result<Vec<Server>>;
  async fn update_server(&self, id: Uuid, dto: dtos::UpdateServerDto) -> Result<Server>;
  async fn save_key_pair(&self, dto: dtos::CreateKeyPairDto) -> Result<KeyPair>;
  async fn get_keys_by_server_id(&self, id: Uuid) -> Result<KeyPair>;
}