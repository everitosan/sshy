use std::str::FromStr;

use uuid::Uuid;
use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};
use crate::{
  ssh::domain::{
    group::Group,
    server::Server
  },
  ssh::dtos::{
    CreateGroupDto,
    UpdateGroupDto
  },
  error::Result
};


pub async fn get_all(pool: &Pool<Sqlite>, id: &Option<Uuid>) -> Result<Vec<Group>> {
  let rows;
  let mut query = String::from(r#"
    SELECT 
      g.id, g.parent_id, g.name, s.id, s.name, s.hostname, s.port
    FROM 
      sshy_group g 
    LEFT JOIN 
      sshy_server s ON s.group_id = g.id 
    WHERE g.parent_id
  "#);

  if let Some(group_id) = id {
    query += "= ?"; 
    rows = sqlx::query(&query).bind(group_id.to_string()).fetch_all(pool).await?;
  
  } else {
    query += " IS NULL";
    rows = sqlx::query(&query).fetch_all(pool).await?;
  }

  let res: Vec<Group> = groups_servers_from_row(&rows);
  Ok(res)
}

pub async fn create(pool: &Pool<Sqlite>, dto: CreateGroupDto) -> Result<Group> {
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
    .fetch_one(pool)
    .await?;

  return Ok(group_from_row(&row))

}

pub async fn get_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Option<Group>> {
  let q = r#"
    SELECT 
      g.id, g.parent_id, g.name, s.id, s.name, s.hostname, s.port
    FROM 
      sshy_group g 
    LEFT JOIN 
      sshy_server s ON s.group_id = g.id 
    WHERE g.id = ?"#;

  let rows = sqlx::query(q).bind(id.to_string()).fetch_all(pool).await?;
  let res: Vec<Group> = groups_servers_from_row(&rows);

  if let Some(first_group) = res.get(0) {
    return Ok(Some(first_group.clone()))
  }
  Ok(None)
}

pub async fn update(pool: &Pool<Sqlite>, id: uuid::Uuid, dto: UpdateGroupDto) -> Result<Group> {
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
    .fetch_one(pool)
    .await?;

  return Ok(group_from_row(&row))
}

// Aux functions

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
        port: row.get(6)
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