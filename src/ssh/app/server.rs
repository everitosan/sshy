use std::{fs::{create_dir_all, File}, io::{Read, Write}, path::PathBuf, process::{Command, Stdio}};


use log::debug;
use uuid::Uuid;

use crate::{
  error::{Error, Result},
  ssh::{domain::{Server, SshStore}, dtos::{CreateKeyPairDto, CreateServerDto}}
};

const REGISTER_KEY_SH: &str = include_str!("register_key.sh");

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

  let server = store.create_server(dto).await?;

  let mut key_path = ssh_path.clone();
  key_path.push("sshy_keys");

  let keypair = match create_keys(&keypair_id.to_string(), &key_path, password) {
    Ok((public, private)) => {
      let keypair = CreateKeyPairDto {
        id: keypair_id,
        server_id: server_id,
        public, private
      };
      store.save_key_pair(keypair).await?
    },
    Err(e) => {
      return Err(Error::Internal(format!("ssh-keygen process {}", e)))
    }
  };

  let variables: Option<Vec<String>> = Some(vec![format!("PUBKEY='{}'", keypair.public.trim())]);
  // let script_path = get_or_greate_register_script(ssh_path)?;
  remote_execute(&server, REGISTER_KEY_SH, variables)?;

  return  Ok(server)
}


pub fn remote_execute(server: &Server, script: &str, variables: Option<Vec<String>>) -> Result<String> {
  let dst = format!("{}@{}", server.user, server.hostname);
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