mod input;

use log::debug;
use sqlx::SqlitePool;


use sshy::{
  config::Config,
  ssh::{domain::SshStore, infra::repository::{DBCreateResutl, SqliteStore}},
};


#[async_std::main]
async fn main() -> Result<(), ()> {
  env_logger::init();

  let config = match Config::exists() {
    Ok(exist) => {
      if exist {
        // Read configuration
        Config::read().expect("error")
      } else {
        let dto = input::config::ask().unwrap();
        Config::create(&dto).expect("error")
      }
    },
    Err(e) => {
      println!("Error {}", e);
      return Ok(())
    }
  };

  // let pass = match input::password::ask(config.db_name.exists()) {
  //   Some(p) => p,
  //   None => {
  //     return Ok(())
  //   }
  // };

  // DB Instance
  let db_res = match SqliteStore::try_create(&config.db_name).await {
    Ok(d) => d,
    Err(e) => {
      println!("{}", e);
      return Ok(());
    }
  };

  let pool = SqlitePool::connect(&config.db_name.to_str().unwrap()).await.unwrap();
  let sqlite_repo = SqliteStore::new(&pool);

  match db_res {
    DBCreateResutl::Created => {
      sqlite_repo.initialize().await.unwrap();
      debug!("Database created");
    },
    DBCreateResutl::Existed => {
      debug!("Database already exists");
    }
  };

  Ok(())
}
