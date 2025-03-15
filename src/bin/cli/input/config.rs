use inquire::Text;
use sshy::{config::CreateConfigDto, error:: Result};

pub fn ask() -> Result<CreateConfigDto> {
  let mut ssh_dir = dirs::home_dir().unwrap();
  ssh_dir.push(".ssh");

  let db_name = Text::new("Database name").with_default("sshy.db").prompt().unwrap();
  let ssh_path = Text::new("Ssh path").with_default(ssh_dir.as_os_str().to_str().unwrap()).prompt().unwrap();
  Ok(CreateConfigDto {
    db_name,
    ssh_path
  })
}