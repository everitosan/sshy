use std::path::PathBuf;

use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

use crate::error::{Result, Error};
use super::super::domain::SshStore;

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

  fn create_group(dto: crate::ssh::dtos::CreateServerDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }

  fn select_groups(id: Option<uuid::Uuid>) -> crate::error::Result<Vec<crate::ssh::domain::Server>> {
    todo!()
  }

  fn update_group(id: uuid::Uuid, dto: crate::ssh::dtos::CreateServerDto) -> crate::error::Result<crate::ssh::domain::Group> {
    todo!()
  }

  fn create_server(dto: crate::ssh::dtos::CreateServerDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }

  fn select_servers(id: uuid::Uuid) -> crate::error::Result<Vec<crate::ssh::domain::Server>> {
    todo!()
  }

  fn update_server(id: uuid::Uuid, dto: crate::ssh::dtos::CreateServerDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }

  fn update_server_keys(id: uuid::Uuid, dto: crate::ssh::dtos::KeyDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }
}

