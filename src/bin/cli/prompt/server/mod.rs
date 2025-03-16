use colorize::AnsiColor;
use inquire::Text;
use sshy::error::Result;

pub mod options;

pub struct ServerPrompt {
  pub name: String,
  pub host: String,
  pub port: u64,
  pub user: String,
}

pub fn ask() -> Result<ServerPrompt>{
  let name_message = "Server name".green();
  let host_message = "Host address".green();
  let port_message = "Port".green();
  let user_message = "User".green();


  let s = ServerPrompt {
    name: Text::new(&name_message).prompt()?,
    host: Text::new(&host_message).prompt()?,
    port: inquire::prompt_u64(port_message)?,
    user: Text::new(&user_message).prompt()?
  };

  Ok(s)
}