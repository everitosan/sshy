use colorize::AnsiColor;
use inquire::Text;
use sshy::error::Result;

pub mod options;
pub mod transform;

pub struct ServerPrompt {
  pub name: String,
  pub host: String,
  pub port: u32,
}

pub fn ask() -> Result<ServerPrompt>{
  let name_message = "Server name".green();
  let host_message = "Host address".green();
  let port_message = "Port".green();


  let s = ServerPrompt {
    name: Text::new(&name_message).prompt()?,
    host: Text::new(&host_message).prompt()?,
    port: inquire::prompt_u32(port_message)?
  };

  Ok(s)
}