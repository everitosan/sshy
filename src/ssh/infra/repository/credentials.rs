use std::str::FromStr;

use uuid::Uuid;
use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};
use crate::{
  ssh::domain::credentials::Credentials,
  ssh::dtos::CreateCredentialsDto,
  error::Result
};



pub async fn save(pool: &Pool<Sqlite>, dto: CreateCredentialsDto) -> Result<Credentials> {
  let query = r#"
    INSERT INTO sshy_credentials 
      (id, name, user, public_key, private_key) 
    VALUES 
      (?, ?, ?, ?, ?)
    RETURNING
      id, name, user, public_key, private_key
  "#;

  let row = sqlx::query(query)
    .bind(dto.id.to_string())
    .bind(dto.name)
    .bind(dto.user)
    .bind(dto.public)
    .bind(dto.private)
    .fetch_one(pool)
    .await?;

  let relation_q = r#"
    INSERT INTO sshy_server_credentials
      (server_id, credentials_id)
    VALUES 
      (?, ?)
  "#;

  sqlx::query(relation_q)
    .bind(dto.server_id.to_string())
    .bind(dto.id.to_string())
    .execute(pool)
    .await?;

  Ok(credentials_from_row(&row))
}

pub async fn get_by_server_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Vec<Credentials>> {
    let q = r#"
    SELECT 
      c.id, c.name, c.user, c.public_key, c.private_key
    FROM 
      sshy_server_credentials sc
    LEFT JOIN 
      sshy_credentials c ON c.id = sc.credentials_id
    WHERE
      sc.server_id = ?
  "#;

  let rows = sqlx::query(q)
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

  let res = rows.iter().map(|r| credentials_from_row(r) ).collect();

  Ok(res)
}


fn credentials_from_row(row: &SqliteRow) -> Credentials {
  let mut credentials = Credentials::default();
  credentials.id = Uuid::from_str(row.get(0)).unwrap();
  credentials.name = row.get(1);
  credentials.user = row.get(2);
  credentials.public = row.get(3);
  credentials.private = row.get(4);

  credentials
}