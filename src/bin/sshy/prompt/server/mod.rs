use std::str::FromStr;

use std::path::PathBuf;
use colorize::AnsiColor;
use inquire::Text;
use sshy::error::{Result, Error};

pub mod options;
pub mod transform;

pub struct ServerPrompt {
  pub name: String,
  pub host: String,
  pub port: u32,
}

pub fn ask() -> Result<ServerPrompt>{
  let name_message = "Server name:".green();
  let host_message = "Host address:".green();
  let port_message = "Port:".green();


  let s = ServerPrompt {
    name: Text::new(&name_message).prompt()?,
    host: Text::new(&host_message).prompt()?,
    port: inquire::prompt_u32(port_message)?
  };

  Ok(s)
}

pub fn ask_script() -> Result<PathBuf> {
  let message = "Script path:".green();
  let mut script_path_str = Text::new(&message).prompt()?;
  script_path_str = script_path_str.trim().to_owned();

  let script_path = PathBuf::from_str(&script_path_str)
    .map_err(|e| Error::Internal(format!("Could not create PathBuff, {}", e)))?;

  if !script_path.exists() {
    return Err(Error::FsError(format!("Script {} does not exist", script_path_str)))
  }
  
  Ok(script_path)
}