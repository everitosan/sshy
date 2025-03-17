use uuid::Uuid;


#[derive(Default)]
pub struct CreateKeyPairDto {
  pub id: Uuid,
  pub server_id: Uuid,
  pub public: String,
  pub private: String
}

#[derive(Default)]
pub struct CreateServerDto {
  pub id: Uuid,
  pub group_id: Uuid,
  pub name: String,
  pub host: String,
  pub port: u32,
  pub user: String
}


#[derive(Default)]
pub struct CreateGroupDto {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub name: String
}

#[derive(Default)]
pub struct UpdateGroupDto {
  pub parent_id: Uuid,
  pub name: String
}