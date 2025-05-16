use std::{fs::{self, File}, io::Write, path::PathBuf, str::FromStr};

use dirs;
use crate::error::{Result, Error};

static CONFIG_FILE: &'static str = ".sshy.json";

#[derive(Debug)]
pub struct Config {
  pub db_name: PathBuf,
  pub ssh_path: PathBuf
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize)] 
pub struct CreateConfigDto {
  pub db_name: String,
  pub ssh_path: String
}

fn get_config_dir() -> Result<PathBuf> {
  let mut config_dir = match dirs::config_dir() {
    Some(p) => p,
    None => {
      return Err(Error::FsError("could not determine config dir".to_owned()))
    }
  };
  config_dir.push("sshy");
  Ok(config_dir)
}

fn get_config_file() -> Result<PathBuf> {
  let mut config_file = get_config_dir()?;
  config_file.push(CONFIG_FILE);
  Ok(config_file)
}

impl Config {
  pub fn exists() -> Result<bool> {
    let path = get_config_file()?;
    Ok(path.exists())
  }

  pub fn create(dto: &CreateConfigDto) -> Result<Self> {
    // create path for db file
    let mut db_file = get_config_dir()?;
    db_file.push(&dto.db_name);
    
    // modify config to save db at config file
    let real_config = CreateConfigDto {
      ssh_path: dto.ssh_path.clone(),
      db_name: format!("{}", db_file.to_str().unwrap())
    };

    let config_file_path = get_config_file()?;
    // create config dir
    fs::create_dir_all(&config_file_path.parent().unwrap())
      .map_err(|e| Error::FsError(format!("could not create config dir: {}", e)))?;
    // create and write .sshy.json file
    let mut file = File::create(config_file_path)
      .map_err(|e| Error::FsError(format!("could not create config file: {}", e)))?;
  
    let data: String = serde_json::to_string(&real_config).unwrap();
    file.write_all(data.as_bytes())
      .map_err(|e| Error::FsError(format!("could not create config file: {}", e)))?;

    Ok(Config {
      db_name: db_file,
      ssh_path: PathBuf::from_str(&dto.ssh_path).unwrap()
    })
  }

  pub fn read() -> Result<Self> {
    let path = get_config_file()?;

    let confg_str = fs::read_to_string(path)
      .map_err(|e| Error::FsError(format!("Could not read config file {}", e )))?;
    let dto: CreateConfigDto = serde_json::from_str(&confg_str)
      .map_err(|e| Error::FsError(format!("could not parse {}", e)))?;

    Ok(Config {
      db_name: PathBuf::from_str(&dto.db_name).unwrap(),
      ssh_path: PathBuf::from_str(&dto.ssh_path).unwrap()
    })
  }
}
