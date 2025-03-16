use std::{path::PathBuf, str::FromStr};

use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, Pool, Row, Sqlite};
use uuid::Uuid;

use crate::error::{Error, Result};
use super::super::domain::SshStore;

use crate::ssh::{
  dtos::{
    CreateGroupDto, UpdateGroupDto,
    CreateServerDto,
    CreateKeyPairDto
  },
  domain:: {
    Server, Group, KeyPair
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

  async fn create_group(&self, dto: CreateGroupDto) -> Result<Group> {
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

  async fn list_groups(&self, id: &Option<uuid::Uuid>) -> Result<Vec<Group>> {
    let rows;
    if let Some(group_id) = id {
      let q = r#"SELECT id, parent_id, name FROM sshy_group WHERE parent_id = ?"#;
      rows = sqlx::query(q).bind(group_id.to_string()).fetch_all(self.pool).await?;
    
    } else {
      let q = r#"SELECT id, parent_id, name FROM sshy_group WHERE parent_id IS NULL"#;
      rows = sqlx::query(q).fetch_all(self.pool).await?;
    }

    let res: Vec<Group> = rows.iter().map(|r| group_from_row(r)).collect();
    Ok(res)
  }

  async fn get_group_by_id(&self, id: Uuid) -> Result<Option<Group>> {
    let q = r#"SELECT id, parent_id, name FROM sshy_group WHERE id = ?"#;
    if let Some(row) = sqlx::query(q).bind(id.to_string()).fetch_optional(self.pool).await? {
      let g = group_from_row(&row);
      return Ok(Some(g));
    } 
    Ok(None)
  }

  fn update_group(id: uuid::Uuid, dto: UpdateGroupDto) -> Result<Group> {
    todo!()
  }

  async fn create_server(&self, dto: CreateServerDto) -> Result<Server> {
    let query = r#"
      INSERT INTO sshy_server 
        (id, group_id, name, hostname, port, user) 
      VALUES 
        (?, ?, ?, ?, ?, ?)
      RETURNING
        id, group_id, name, hostname, port, user
    "#;

    let row = sqlx::query(query)
      .bind(dto.id.to_string())
      .bind(dto.group_id.to_string())
      .bind(dto.name)
      .bind(dto.host)
      .bind(dto.port)
      .bind(dto.user)
      .fetch_one(self.pool)
      .await?;

    return Ok(server_from_row(&row))
  }

  async fn list_servers(&self, group_id: uuid::Uuid) -> Result<Vec<Server>> {
    let q = r#"
      SELECT 
        id server, name, hostname, port, user
      FROM 
        sshy_server
      WHERE
        s.id = ?
    "#;
    let rows= sqlx::query(q).bind(group_id.to_string()).fetch_all(self.pool).await?;
    let res: Vec<Server> = rows.iter().map(|r| server_from_row(r)).collect();

    Ok(res)
  }

  fn update_server(id: uuid::Uuid, dto: crate::ssh::dtos::CreateServerDto) -> Result<Server> {
    todo!()
  }

  async fn save_key_pair(&self, dto: CreateKeyPairDto) -> Result<KeyPair> {
    let query = r#"
      INSERT INTO sshy_key_pair 
        (id, server_id, public_key, private_key) 
      VALUES 
        (?, ?, ?, ?)
      RETURNING
        id, server_id, public_key, private_key
    "#;

    let row = sqlx::query(query)
      .bind(dto.id.to_string())
      .bind(dto.server_id.to_string())
      .bind(dto.public)
      .bind(dto.private)
      .fetch_one(self.pool)
      .await?;

    Ok(keypair_from_row(&row))
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

fn server_from_row(row: &SqliteRow) -> Server {
  let mut s = Server::default();
  s.id = Uuid::from_str(row.get(0)).unwrap();
  s.group_id = Uuid::from_str(row.get(1)).unwrap();
  s.name = row.get(2);
  s.hostname = row.get(3);
  s.port = row.get(4);
  s.user = row.get(5);
  s
}

fn keypair_from_row(row: &SqliteRow) -> KeyPair {
  let mut key_pair = KeyPair::default();
  key_pair.id = Uuid::from_str(row.get(0)).unwrap();
  key_pair.server_id = Uuid::from_str(row.get(1)).unwrap();
  key_pair.public = row.get(2);
  key_pair.private = row.get(3);

  key_pair
}