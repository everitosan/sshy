use uuid::Uuid;
use async_trait::async_trait;
use crate::error::Result;

use crate::ssh::dtos;


#[derive(Default, Debug, Clone)]
pub struct Credentials {
  pub id: Uuid,
  pub name: String,
  pub user: String,
  pub public: String,
  pub private: String
}


#[async_trait]
pub trait SshyCredentialsRepo {
  async fn save_credentials(&self, dto: dtos::CreateCredentialsDto) -> Result<Credentials>;
  async fn get_credentials_by_server_id(&self, id: Uuid) -> Result<Vec<Credentials>>;
}