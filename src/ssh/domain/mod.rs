use std::path::PathBuf;
use uuid::Uuid;
use async_trait::async_trait;

use crate::error::Result;
use super::dtos;

#[derive(Default)]
pub struct Key {
  pub id: Uuid,
  pub server_id: String,
  pub file: Option<PathBuf>,
  pub content: String,
}

#[derive(Default)]
pub struct Server {
  pub id: Uuid,
  pub group_id: Uuid,
  pub name: String,
  pub hostname: String,
  pub port: u8,
  pub user: String,
  pub private_key: Key,
  pub public_key: Key
}

#[derive(Default)]
pub struct Group {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub name:  String,
  pub group: Option<Vec<Group>>,
  pub servers: Option<Vec<Server>>
}

#[async_trait]
pub trait SshStore {
  async fn initialize(&self) -> Result<()>;
  async fn create_group(&self, dto: dtos::CreateGroupDto) -> Result<Group>;
  async fn list_groups(&self, id: Option<Uuid>) -> Result<Vec<Group>>;
  fn update_group(id: Uuid, dto: dtos::UpdateGroupDto) -> Result<Group>;
  fn create_server(dto: dtos::CreateServerDto) -> Result<Server>;
  fn list_servers(id: Uuid) -> Result<Vec<Server>>;
  fn update_server(id: Uuid, dto: dtos::CreateServerDto) -> Result<Server>;
  fn update_server_keys(id: Uuid, dto: dtos::KeyDto) -> Result<Server>;
}