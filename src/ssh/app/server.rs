use std::{io::Write, path::PathBuf, process::{Command, Stdio}};


use log::debug;
use uuid::Uuid;

use crate::{
  error::{Error, Result},
  ssh::{domain::{
    credentials::{Credentials, SshyCredentialsRepo}, 
    group::SshyGroupRepo, 
    server::{Server, SshyServerRepo}
  }, dtos}
};

use super::credentials;


pub struct CreateServerDto {
  pub name: String,
  pub group_id: Uuid,
  pub host: String,
  pub port: u32,
}

pub struct CreateCredentialsDto {
  pub user: String,
  pub private_key: String,
  pub public_key: String
}


pub async fn create<T: SshyServerRepo + SshyGroupRepo + SshyCredentialsRepo>(store: &T, server_data: CreateServerDto) -> Result<Server> {
  let server_id = Uuid::new_v4();

  if let None = store.get_group_by_id(server_data.group_id).await? {
    return Err(Error::Integrity(format!("Group {} does not exists", server_data.group_id)) );
  }
  
  let server_dto = dtos::CreateServerDto {
    id: server_id,
    name: server_data.name,
    group_id: server_data.group_id,
    host: server_data.host,
    port: server_data.port
  };

  store.create_server(server_dto).await
}

// Creates an SSH connection to server
pub async fn connect(server: &Server, credentials: &Credentials, ssh_path: &PathBuf) -> Result<()> {

  let dst = format!("{}@{}", credentials.user, server.hostname);

  let key_path = credentials::ensure_private_key(ssh_path, credentials)?;

  let child = Command::new("ssh")
    .arg(&dst)
    .arg("-p")
    .arg(format!("{}", server.port))
    .arg("-i")
    .arg(format!("{}", key_path.to_str().unwrap().trim()))
    .spawn()
    .map_err(|e| Error::Internal(format!("could not create remote command {}", e)))?;

  let output = child.wait_with_output()
    .map_err(|e| Error::Internal(format!("could connect to server {}", e)))?;

  if !output.status.success() {
    return Err(Error::Command { bin: "ssh".to_owned(), message: format!("stderr of remote connection is {}", String::from_utf8_lossy(&output.stderr)) })
  }

  Ok(())
}

pub fn remote_execute(server: &Server, user: &str, script: &str, variables: Option<Vec<String>>) -> Result<String> {
  let dst = format!("{}@{}", user, server.hostname);
  let train_vars = match variables {
    Some(vars) => {
      let train = vars.join(" ");
      format!("{}", train.trim())
    },
    None => {
      String::from("")
    }
  };

  debug!("ssh {} -p {} {} bash -s ", &dst, server.port, &train_vars);

  let mut child = Command::new("ssh")
    .arg(&dst)
    .arg("-p")
    .arg(format!("{}", server.port))
    .arg(format!("{} bash -s", &train_vars))
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .map_err(|e| Error::Internal(format!("could not create remote command {}", e)))?;

  if let Some(mut stdin) = child.stdin.take() {
    stdin.write_all(script.as_bytes())
      .map_err(|e| Error::Internal(format!("error trying to direct stdin {}", e)))?;
  }

  let output = child.wait_with_output()
    .map_err(|e| Error::Internal(format!("could not execute remotely {}", e)))?;

  if !output.status.success() {
    return Err(Error::Command { bin: "ssh".to_owned(), message: format!("stderr of remote execution is {}", String::from_utf8_lossy(&output.stderr)) })
  }

  let ouput_ok = String::from_utf8_lossy(&output.stdout);

  Ok(format!("{}", ouput_ok))
}

