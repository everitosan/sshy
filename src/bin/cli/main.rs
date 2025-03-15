mod input;
mod parser;


use input::group::groups_as_vec;
use inquire::Select;
use log::debug;
use sqlx::SqlitePool;

use clap::{Parser, Subcommand};

use sshy::{
  config::Config,
  ssh::{app::{create_group, list_groups}, domain::SshStore, infra::repository::{DBCreateResutl, SqliteStore}},
};
use parser::group::{GroupActions, GroupCommand};


#[derive(Debug, Parser)]
#[command(name = "sshy", version = "1.0", about = "ssh connections manager")]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>
}

#[derive(Debug, Subcommand)] 
enum Command {
  /// Actions over a group
  Group(GroupCommand),
  /// Actions over a server
  Server(ServerCommmand)
}



/*
* 🆂🅴🆁🆅🅴🆁
*/
#[derive(Debug, Parser)]
struct ServerCommmand {

}


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


  let cli = Cli::parse();

  if let Some (command) = cli.command {
    match command {
      Command::Group(gc) => {
        match gc.command {
          GroupActions::List => {
            match list_groups(&sqlite_repo, None).await {
              Ok(groups) => {
  
              },
              Err(e) => {
                panic!("{}", e);
              }
            };
          },
          GroupActions::Create(args) => {
  
          },
          GroupActions::Edit => {
    
          }
        }
      },
      Command::Server(cs) => {
  
      }
    };

  } else {
    // Interactive mode
    match list_groups(&sqlite_repo, None).await {
      Ok(groups) => {
        let mut options = groups_as_vec(&groups);
        options.push("-> Create".to_owned());
        let opt = Select::new("Selecciona un grupo, servidor o acción", options).prompt().unwrap();
        match opt.as_str() {
          "-> Create" => {
            let name = input::group::ask_group().unwrap();
             create_group(&sqlite_repo, &name, None).await.unwrap();
          },
          _ => {

          }
        }
      },
      Err(e) => {
        panic!("{}", e);
      }
    };
  }


  Ok(())
}
