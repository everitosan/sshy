pub mod transform;
pub mod options;

use colorize::AnsiColor;
use inquire::Text;
use sshy::error::Result;


pub fn ask_group() -> Result<String> {
  let message = "Group name".green();
  let name = Text::new(&message).prompt()?;
  Ok(name)
}