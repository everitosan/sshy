use uuid::Uuid;

use crate::{
  error::Result,
  ssh::{domain::group::{SshyGroupRepo, Group}, dtos::CreateGroupDto}
};

pub async fn create<T: SshyGroupRepo>(store: &T, name: &str,  parent_group: &Option<Group>) -> Result<Group> {
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

pub async fn get<T: SshyGroupRepo>(store: &T, id: Uuid) -> Result<Option<Group>> {
  store.get_group_by_id(id).await
}

pub async fn list<T: SshyGroupRepo>(store: &T, parent_group: &Option<Group>) -> Result<Vec<Group>> {
  let id: Option<Uuid> = if let Some(parent) = parent_group {
  Some(parent.id.clone())
  } else {
    None
  };

  store.get_all_groups(&id).await
}