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
    let mut query = String::from(r#"
      SELECT 
        g.id, g.parent_id, g.name, s.id, s.name, s.hostname, s.port, s.user
      FROM 
        sshy_group g 
      LEFT JOIN 
        sshy_server s ON s.group_id = g.id 
      WHERE g.parent_id
    "#);

    if let Some(group_id) = id {
      query += "= ?"; 
      rows = sqlx::query(&query).bind(group_id.to_string()).fetch_all(self.pool).await?;
    
    } else {
      query += " IS NULL";
      rows = sqlx::query(&query).fetch_all(self.pool).await?;
    }

    let res: Vec<Group> = groups_servers_from_row(&rows);
    Ok(res)
  }

  async fn get_group_by_id(&self, id: Uuid) -> Result<Option<Group>> {
    let q = r#"
      SELECT 
        g.id, g.parent_id, g.name, s.id, s.name, s.hostname, s.port, s.user
      FROM 
        sshy_group g 
      LEFT JOIN 
        sshy_server s ON s.group_id = g.id 
      WHERE g.id = ?"#;

    let rows = sqlx::query(q).bind(id.to_string()).fetch_all(self.pool).await?;
    let res: Vec<Group> = groups_servers_from_row(&rows);

    if let Some(first_group) = res.get(0) {
      return Ok(Some(first_group.clone()))
    }
    Ok(None)
  }

  async fn update_group(&self, id: uuid::Uuid, dto: UpdateGroupDto) -> Result<Group> {
    let query = r#"
      UPDATE sshy_group SET
        (name, parent_id) 
      VALUES 
        (?, ?)
      WHERE
        id = ?
      RETURNING
        id, parent_id, name
    "#;

    let row = sqlx::query(query)
      .bind(dto.name)
      .bind(dto.parent_id.to_string())
      .bind(id.to_string())
      .fetch_one(self.pool)
      .await?;

    return Ok(group_from_row(&row))
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


fn groups_servers_from_row(rows: &Vec<SqliteRow>) -> Vec<Group> {

  let mut res: Vec<Group> = vec![];
  let mut servers: Vec<Server> = vec![];
  let mut last_group = Group::default();

  for row in rows {
    let group_id: String = row.get(0);
    if group_id != last_group.id.to_string() {
      if last_group.id.to_string() != Uuid::default().to_string()  {
        last_group.servers = servers;
        servers = vec![];
        res.push(last_group);
      }
      // create group
      last_group = group_from_row(row);
    }
    // Work over servers
    let server_id: String = row.get(3);
    if !server_id.is_empty() {
      let tmp_server = Server {
        id: Uuid::from_str(row.get(3)).unwrap(), 
        group_id: last_group.id.clone(),
        name: row.get(4),
        hostname: row.get(5),
        port: row.get(6),
        user: row.get(7)
      };
      servers.push(tmp_server);
    }
  }

  if last_group.id.to_string() != Uuid::default().to_string() {
    last_group.servers = servers;
    res.push(last_group);    
  }

  res
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