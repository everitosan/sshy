use std::{path::PathBuf, str::FromStr};

use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, Pool, Row, Sqlite};
use uuid::Uuid;

use crate::error::{Error, Result};
use super::super::domain::SshStore;

use crate::ssh::{
  dtos::{
    CreateGroupDto, CreateServerDto, UpdateGroupDto
  },
  domain:: {
    Server, Group
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

  async fn create_group(&self, dto: CreateGroupDto) -> crate::error::Result<Group> {
    let parent_id: Option<String> = match dto.parent_id {
      Some(s) => Some(s.to_string()),
      None => None
    };

    let query = r#"
      INSERT INTO sshy_group 
        (id, parent_id, name) 
      VALUES 
        (?, ?, ?)
      RETURNING
        id, parent_id, name
    "#;

    let row = sqlx::query(query)
      .bind(dto.id.to_string())
      .bind(parent_id)
      .bind(dto.name)
      .fetch_one(self.pool)
      .await?;

    return Ok(group_from_row(&row))

  }

  async fn list_groups(&self, id: Option<uuid::Uuid>) -> crate::error::Result<Vec<Group>> {
    let rows;
    if let Some(group_id) = id {
      let q = r#"SELECT (id, parent_id, name) FROM sshy_group WHERE parent_id = ?"#;
      rows = sqlx::query(q).bind(group_id.to_string()).fetch_all(self.pool).await?;
    
    } else {
      let q = r#"SELECT (id, parent_id, name) FROM sshy_group"#;
      rows = sqlx::query(q).fetch_all(self.pool).await?;
    }

    let res: Vec<Group> = rows.iter().map(|r| group_from_row(r)).collect();
    Ok(res)
  }

  fn update_group(id: uuid::Uuid, dto: UpdateGroupDto) -> crate::error::Result<Group> {
    todo!()
  }

  fn create_server(dto: CreateServerDto) -> crate::error::Result<Server> {
    todo!()
  }

  fn list_servers(id: uuid::Uuid) -> crate::error::Result<Vec<crate::ssh::domain::Server>> {
    todo!()
  }

  fn update_server(id: uuid::Uuid, dto: crate::ssh::dtos::CreateServerDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }

  fn update_server_keys(id: uuid::Uuid, dto: crate::ssh::dtos::KeyDto) -> crate::error::Result<crate::ssh::domain::Server> {
    todo!()
  }
}

fn group_from_row(row: &SqliteRow) -> Group {
  let mut g = Group::default();
  g.id = Uuid::from_str(row.get(0)).unwrap();
  g.parent_id = match row.get(1) {
    Some(p) => {
      Some(Uuid::from_str(p).unwrap())
    },
    None => None
  };
  g.name = row.get(2);
  g
}