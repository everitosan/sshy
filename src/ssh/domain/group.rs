use uuid::Uuid;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

use super::server::Server;
use super::super::dtos;
use crate::error::Result;


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub name:  String,
  pub group: Option<Vec<Group>>,
  pub servers: Vec<Server>
}


#[async_trait]
pub trait SshyGroupRepo {
  async fn get_all_groups(&self, id: &Option<Uuid>) -> Result<Vec<Group>>;
  async fn get_group_by_id(&self, id: Uuid) -> Result<Option<Group>>;
  async fn create_group(&self, dto: dtos::CreateGroupDto) -> Result<Group>;
  async fn update_group(&self, id: Uuid, dto: dtos::UpdateGroupDto) -> Result<Group>; 
}