pub mod error;
pub mod ssh;
pub mod config;

use config::{Config, CreateConfigDto};
use dirs;
use inquire::Text;
use log::debug;

fn main() {
  env_logger::init();
  
  let config = if let Ok(exist) = Config::exists() {
    if !exist {
      let mut ssh_dir = dirs::home_dir().unwrap();
      ssh_dir.push(".ssh");
      
      let db_name = Text::new("Database name").with_default("sshy.db").prompt().unwrap();
      let ssh_path = Text::new("Ssh path").with_default(ssh_dir.as_os_str().to_str().unwrap()).prompt().unwrap();
      let dto = CreateConfigDto {
        db_name,
        ssh_path
      };

      Config::create(&dto).expect("error")
    } else {
      Config::read().expect("error")
    }
    
  } else {
    panic!("");
  };

  debug!("{:?}", config);
  println!("Hello, world!");
}
