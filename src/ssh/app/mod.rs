use uuid::Uuid;

use crate::error::Result;
use super::{domain::{Group, SshStore}, dtos::CreateGroupDto};


pub async fn create_group<T: SshStore>(store: &T, name: &str,  parent_group: &Option<Group>) -> Result<Group> {
  let parent_id: Option<Uuid> = if let Some(parent) = parent_group {
    Some(parent.id.clone())
  } else {
    None
  };
  let id = Uuid::new_v4();
  let dto = CreateGroupDto {
    id,
    parent_id,
    name: name.to_owned()
  };
  
  store.create_group(dto).await
}

pub async fn get_group<T: SshStore>(store: &T, id: Uuid) -> Result<Group> {
  store.get_by_id(id).await
}

pub async fn list_groups<T: SshStore>(store: &T, parent_group: &Option<Group>) -> Result<Vec<Group>> {

  let id: Option<Uuid> = if let Some(parent) = parent_group {
  Some(parent.id.clone())
  } else {
    None
  };

  store.list_groups(&id).await
}