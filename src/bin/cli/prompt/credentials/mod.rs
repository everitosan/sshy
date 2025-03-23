use std::{fs, path::PathBuf};
use colorize::AnsiColor;
use inquire::Text;
use sshy::error::Result;

pub mod transform;

#[derive(Default)]
pub struct CredentialsPrompt {
  pub name: String,
  pub user: String,
  pub private_key: Option<String>,
  pub public_key: Option<String>
}

pub fn ask() -> Result<CredentialsPrompt>{
  let name_message = "Credentials name:".green();
  let user_message = "User:".green();
  let public_message = "Path of public key (optional):".green();
  let private_message = "Path of private key (optional):".green();

  let mut c = CredentialsPrompt::default();
  c.name = Text::new(&name_message).with_default("default").prompt()?;
  c.user = Text::new(&user_message).with_default("root").prompt()?;
  c.public_key = get_key_str(&public_message);
  c.private_key = get_key_str(&private_message);

  Ok(c)
}

fn get_key_str(message: &str) -> Option<String> {
  let path_str = match Text::new(message).prompt() {
    Ok(k) => { k },
    Err(_) => { return None}
  };

  let path = PathBuf::from(&path_str);

  if !path.exists() {
    return None
  }

  let file_data = match fs::read_to_string(path) {
    Ok(d) => {d},
    Err(_e) => {
      return None
    }
  };

  Some(file_data)
}