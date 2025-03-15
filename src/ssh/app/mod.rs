use std::str::FromStr;

use uuid::Uuid;

use crate::error::{Result, Error};
use super::{domain::{Group, SshStore}, dtos::CreateGroupDto};


pub async fn create_group<T: SshStore>(store: &T, name: &str, parent_id: Option<Uuid>) -> Result<Group> {
  let id = Uuid::new_v4();
  let dto = CreateGroupDto {
    id,
    parent_id,
    name: name.to_owned()
  };
  
  store.create_group(dto).await
}

pub async fn list_groups<T: SshStore>(store: &T, id_str: Option<String>) -> Result<Vec<Group>> {
  let id: Option<Uuid> = match id_str {
    Some(str) => {
      let id = Uuid::from_str(&str)
        .map_err(|e| Error::Integrity(format!("{}", e)) )?;
      Some(id)
    },
    None => None
  };

  store.list_groups(id).await
}