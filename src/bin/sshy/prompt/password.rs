use colorize::AnsiColor;
use inquire::Password;
use sshy::error::Result;

pub fn ask(exists: bool) -> Result<String> {
  let p;
  let message = "Type secret password".green();
  if exists {
    p = Password::new(&message).without_confirmation().prompt()?;
  } else {
    p = Password::new(&message).prompt()?;
  }
  Ok(p)
}