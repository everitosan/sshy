pub mod group;
pub mod server;
pub mod credentials;

use std::path::PathBuf;


use uuid::Uuid;
use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

use crate::error::{Error, Result};

use crate::ssh::{
  dtos::{
    CreateGroupDto, 
    UpdateGroupDto,
    CreateServerDto,
    UpdateServerDto,
    CreateCredentialsDto
  },
  domain:: {
    SshStore,
    group::{SshyGroupRepo, Group},
    server::{SshyServerRepo, Server},
    credentials::{SshyCredentialsRepo, Credentials}
  }
};


const SCHEMA: &str = include_str!("schema.sql");

pub struct SqliteStore<'a> {
  pub pool: &'a Pool<Sqlite>
}

pub enum DBCreateResutl {
  Existed,
  Created
}

fn get_db_name(db_path: &PathBuf) -> Result<String> {
  let mut str = String::from("sqlite://");
  if let Some (p) = db_path.to_str() {
    str = format!("{}{}", str, p);
    return Ok(str)
  }
  return Err(Error::Internal( "error trying to get name".to_owned() ))
}

impl <'a> SqliteStore <'a> {
  pub async fn try_create(db_file: &PathBuf) -> Result<DBCreateResutl> {
    let db_str = get_db_name(db_file)?;
    if !Sqlite::database_exists(&db_str).await.unwrap_or(false) {
      if let Err(e) = Sqlite::create_database(&db_str).await {
        return Err(Error::DB(format!("{}", e)))
      }
      return Ok(DBCreateResutl::Created);
    }
    Ok(DBCreateResutl::Existed)
  }

  pub fn new(pool: &'a Pool<Sqlite>) -> Self{
    SqliteStore {
      pool
    }
  }
}

#[async_trait]
impl <'a> SshStore for SqliteStore<'a> {
  async fn initialize(&self) -> Result<()> {
    sqlx::query(SCHEMA).execute(self.pool).await?;
    Ok(())
  }
}

// Group methods
#[async_trait]
impl <'a> SshyGroupRepo for SqliteStore <'a> {
  async fn create_group(&self, dto: CreateGroupDto) -> Result<Group> {
    group::create(self.pool, dto).await
  }
  async fn get_all_groups(&self, id: &Option<Uuid>) -> Result<Vec<Group>> {
    group::get_all(self.pool, id).await
  }
  async fn get_group_by_id(&self, id: Uuid) -> Result<Option<Group>> {
    group::get_by_id(self.pool, id).await
  }
  async fn update_group(&self, id: Uuid, dto: UpdateGroupDto) -> Result<Group> {
    group::update(self.pool, id, dto).await
  }
}

// Server methods
#[async_trait]
impl <'a> SshyServerRepo for SqliteStore <'a> {
  async fn create_server(&self, dto: CreateServerDto) -> Result<Server> {
    server::create(self.pool, &dto).await
  }
  async fn list_servers(&self, group_id: Uuid) -> Result<Vec<Server>> {
    server::list(self.pool, group_id).await
  }
  async fn update_server(&self, id: Uuid, dto: UpdateServerDto) -> Result<Server> {
    server::update(self.pool, id, dto).await
  }
}


// Credentials methods
#[async_trait]
impl <'a> SshyCredentialsRepo for SqliteStore <'a> {
  async fn save_credentials(&self, dto: CreateCredentialsDto) -> Result<Credentials> {
    credentials::save(self.pool, dto).await
  }

  async fn get_credentials_by_server_id(&self, id: Uuid) -> Result<Vec<Credentials>> {
    credentials::get_by_server_id(self.pool, id).await
  }
}