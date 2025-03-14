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
    let path = get_config_file()?;
    // create dir
    fs::create_dir_all(&path.parent().unwrap())
      .map_err(|e| Error::FsError(format!("could not create config dir: {}", e)))?;
    println!("{:?}", path);
    // create and write file
    let mut file = File::create(path)
      .map_err(|e| Error::FsError(format!("could not create config file: {}", e)))?;
    let data: String = serde_json::to_string(&dto).unwrap();
    file.write_all(data.as_bytes())
      .map_err(|e| Error::FsError(format!("could not create config file: {}", e)))?;
    Ok(Config {
      db_name: PathBuf::from_str(&dto.db_name).unwrap(),
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
