use std::fs::{create_dir_all, set_permissions, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::env::consts::OS;

use std::os::unix::fs::PermissionsExt;

use colorize::AnsiColor;
use log::debug;
use uuid::Uuid;


use crate::error::{Error, Result};
use crate::ssh::domain::credentials::{Credentials, SshyCredentialsRepo};
use crate::ssh::domain::server::Server;
use crate::ssh::dtos::CreateCredentialsDto;

use super::server::remote_execute;

pub type SSHKeys = (String, String);

const REGISTER_KEY_SH: &str = include_str!("register_key.sh");

#[derive(Debug)]
pub struct AppCredentialsDto {
  pub name: String,
  pub user: String,
  pub server_id: Uuid,
  pub private_key: Option<String>,
  pub public_key: Option<String>
}

pub async fn get_for_server_id<T: SshyCredentialsRepo>(store: &T, id: Uuid) -> Result<Vec<Credentials>> {
  store.get_credentials_by_server_id(id).await
}

pub async fn create_for_server<T: SshyCredentialsRepo>(store: &T, server: &Server, data: &AppCredentialsDto, key_path: &PathBuf, password: &str) -> Result<Credentials> {
  let keypair_id = Uuid::new_v4();

  let mut public_key = if let Some(public) = &data.public_key {
    public.clone()
  } else {
    "".to_owned()
  };

  let mut private_key = if let Some(private) = &data.private_key {
    private.clone()
  } else {
    "".to_owned()
  };

  if private_key.is_empty() || public_key.is_empty() {
    let keys = create_keys(&keypair_id.to_string(), key_path, password)?;
    public_key = keys.0;
    private_key = keys.1;
  }

  let dto = CreateCredentialsDto {
    id: keypair_id,
    server_id: data.server_id,
    name: data.name.clone(),
    user: data.user.clone(),
    public: public_key,
    private: private_key
  };

  let variables: Option<Vec<String>> = Some(vec![format!("PUBKEY='{}'", dto.public.trim())]);
  remote_execute(&server, &dto.user, REGISTER_KEY_SH, variables)?;
  store.save_credentials(dto).await
}


pub fn get_keys_path(ssh_path: &PathBuf) -> PathBuf{
  let mut new_path = ssh_path.clone();
  new_path.push("sshy_keys");
  new_path
}

pub fn ensure_private_key(ssh_path: &PathBuf, credentials: &Credentials) -> Result<PathBuf> {
  let mut key_path = get_keys_path(ssh_path);
  key_path.push(credentials.id.to_string());

  if !key_path.exists() {
    let key_path_str = &key_path.clone();
    let message = format!("Recreating file {} in {}", key_path_str.to_str().unwrap(), OS);

    println!("{}", message.yellow());

    let mut file = File::create(&key_path)
      .map_err(|e| Error::FsError(format!("could not re-create private key file: {}", e)))?;
      
    file.write_all(credentials.private.as_bytes())
    .map_err(|e| Error::FsError(format!("could not re-write private key file: {}", e)))?;
  
    if OS == "linux" {
      let metadata = file.metadata().unwrap();
      let mut permissions = metadata.permissions();
      permissions.set_mode(0o600);
      set_permissions(&key_path, permissions)
        .map_err(|e| Error::FsError(format!("Could not change file permissions {}", e)))?;
    }
    
  }

  Ok(key_path)
}

fn create_keys(name: &str, ssh_path: &PathBuf, password: &str) -> Result<SSHKeys> {

  let key_path = get_keys_path(ssh_path);

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
    .output()
    .map_err(|e| Error::Internal(format!("could not create key: {}", e)))?;

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