use std::{fs::{create_dir_all, File}, io::Read, path::PathBuf, process::Command};


use log::debug;
use uuid::Uuid;

use crate::{
  error::{Error, Result},
  ssh::{domain::{Server, SshStore}, dtos::{CreateKeyPairDto, CreateServerDto}}
};

pub struct CreaterServer {
  pub name: String,
  pub group_id: Uuid,
  pub host: String,
  pub port: u32,
  pub user: String,
}

pub async fn create<T: SshStore>(store: &T, data: CreaterServer, ssh_path: &PathBuf, password: &str) -> Result<Server> {
  let server_id = Uuid::new_v4();
  let keypair_id = Uuid::new_v4();
  if let None = store.get_group_by_id(data.group_id).await? {
    return Err(Error::Integrity(format!("Group {} does not exists", data.group_id)) );
  }

  let dto = CreateServerDto {
    id: server_id,
    name: data.name,
    group_id: data.group_id,
    host: data.host,
    port: data.port,
    user: data.user
  };

  let saved = store.create_server(dto).await?;

  let mut key_path = ssh_path.clone();
  key_path.push("sshy_keys");

  match create_keys(&keypair_id.to_string(), &key_path, password) {
    Ok((public, private)) => {
      let keypair = CreateKeyPairDto {
        id: keypair_id,
        server_id: server_id,
        public, private
      };
      store.save_key_pair(keypair).await?;
      return  Ok(saved)
    },
    Err(e) => {
      return Err(Error::Internal(format!("ssh-keygen process {}", e)))
    }
  };
}


pub fn create_keys(name: &str, key_path: &PathBuf, password: &str) -> Result<(String, String)> {

  debug!("Key dest dir {:?}", key_path);
  if !key_path.exists() {
    create_dir_all(&key_path)
      .map_err(|e| Error::Internal(format!("could not create keys dir: {}", e)))?;
  }

  // Paths
  let mut private_key_path = key_path.clone();
  private_key_path.push(name);
  let mut public_key_path = private_key_path.clone();
  public_key_path.set_extension("pub");

  debug!("private key path {:?}", private_key_path);
  debug!("public key path {:?}", public_key_path);

  
  let output = Command::new("ssh-keygen")
    .arg("-f")
    .arg(&private_key_path)
    .arg("-t")
    .arg("ed25519")
    .arg("-N")
    .arg(password)
    .output().map_err(|e| Error::Internal(format!("could not create key: {}", e)))?;

  debug!("ssh-key-gen executed");

  if !output.status.success() {
    return Err(Error::Command { bin: "ssh-key".to_owned(), message: format!("{}",  String::from_utf8_lossy(&output.stderr)) })
  }

  // Read private key
  let mut private_key = File::open(private_key_path)
    .map_err(|e| Error::Internal(format!("could not read private key: {}", e)))?;

  let mut private_str = String::new();
  private_key.read_to_string(&mut private_str)
    .map_err(|e| Error::Internal(format!("could not send private key to stream: {}", e)))?;
  
  // Read public key
  let mut public_key = File::open(public_key_path)
    .map_err(|e| Error::Internal(format!("could not read public key: {}", e)))?;

  let mut public_str = String::new();
  public_key.read_to_string(&mut public_str)
    .map_err(|e| Error::Internal(format!("could not send public key to stream: {}", e)))?;

  Ok((public_str, private_str))
}