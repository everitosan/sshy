use uuid::Uuid;


#[derive(Default)]
pub struct CreateCredentialsDto {
  pub id: Uuid,
  pub name: String,
  pub server_id: Uuid,
  pub user: String,
  pub public: String,
  pub private: String
}

#[derive(Default)]
pub struct CreateServerDto {
  pub id: Uuid,
  pub group_id: Uuid,
  pub name: String,
  pub host: String,
  pub port: u32
}


#[derive(Default)]
pub struct UpdateServerDto {
  pub group_id: Uuid,
  pub name: String,
  pub host: String,
  pub port: u32
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