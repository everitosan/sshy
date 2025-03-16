use uuid::Uuid;
use super::domain::KeyType;

#[derive(Default)]
pub struct CreateServerDto {
  pub group_id: Uuid,
  pub name: String,
  pub ip: String,
  pub host: String,
  pub port: u8
}


#[derive(Default)]
pub struct CreateGroupDto {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub name: String
}

#[derive(Default)]
pub struct UpdateGroupDto {
  pub parent_id: Option<Uuid>,
  pub name: Option<String>
}

#[derive(Default)]
pub struct KeyDto {
  pub key_type: KeyType,
  pub content: String,
}