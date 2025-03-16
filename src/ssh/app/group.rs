use uuid::Uuid;

use crate::{
  error::Result,
  ssh::{domain::{Group, SshStore}, dtos::CreateGroupDto}
};

pub async fn create<T: SshStore>(store: &T, name: &str,  parent_group: &Option<Group>) -> Result<Group> {
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

pub async fn get<T: SshStore>(store: &T, id: Uuid) -> Result<Option<Group>> {
  store.get_group_by_id(id).await
}

pub async fn list<T: SshStore>(store: &T, parent_group: &Option<Group>) -> Result<Vec<Group>> {

  let id: Option<Uuid> = if let Some(parent) = parent_group {
  Some(parent.id.clone())
  } else {
    None
  };

  store.list_groups(&id).await
}