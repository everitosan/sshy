use std::str::FromStr;

use uuid::Uuid;
use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};
use crate::{
  ssh::domain::server::Server,
  ssh::dtos::{
    CreateServerDto,
    UpdateServerDto
  },
  error::Result
};

pub async fn create(pool: &Pool<Sqlite>, dto: &CreateServerDto) -> Result<Server> {
  let query = r#"
  INSERT INTO sshy_server 
    (id, group_id, name, hostname, port) 
  VALUES 
    (?, ?, ?, ?, ?)
  RETURNING
    id, group_id, name, hostname, port
"#;

let row = sqlx::query(query)
  .bind(dto.id.to_string())
  .bind(dto.group_id.to_string())
  .bind(&dto.name)
  .bind(&dto.host)
  .bind(&dto.port)
  .fetch_one(pool)
  .await?;

  Ok(server_from_row(&row))
}

pub async fn list(pool: &Pool<Sqlite>, group_id: Uuid) -> Result<Vec<Server>> {
  let q = r#"
  SELECT 
    id, server, name, hostname, port
  FROM 
    sshy_server
  WHERE
    s.id = ?
  "#;
  let rows= sqlx::query(q).bind(group_id.to_string()).fetch_all(pool).await?;
  let res: Vec<Server> = rows.iter().map(|r| server_from_row(r)).collect();

  Ok(res)
}

pub async fn update(pool: &Pool<Sqlite>, id: Uuid, dto: UpdateServerDto) -> Result<Server> {
  let query = r#"
    UPDATE sshy_server SET
      (group_id, name, host, port) 
    VALUES 
      (?, ?, ?, ?, ?)
    WHERE
      id = ?
    RETURNING
      id, group_id, name, host, port
  "#;

  let row = sqlx::query(query)
    .bind(dto.group_id.to_string())
    .bind(dto.name)
    .bind(dto.host)
    .bind(dto.port)
    .bind(id.to_string())
    .fetch_one(pool)
    .await?;

  Ok(server_from_row(&row))
}


fn server_from_row(row: &SqliteRow) -> Server {
  let mut s = Server::default();
  s.id = Uuid::from_str(row.get(0)).unwrap();
  s.group_id = Uuid::from_str(row.get(1)).unwrap();
  s.name = row.get(2);
  s.hostname = row.get(3);
  s.port = row.get(4);
  s
}