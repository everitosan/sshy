mod prompt;
mod parser;

use std::{str::FromStr, thread::sleep, time};

use colorize::AnsiColor;
use inquire::Select;
use log::debug;
use sqlx::SqlitePool;
use clap::{Parser, Subcommand};

use sshy::{
  config::Config,
  ssh::{app::{self, server::CreaterServer}, domain::{Group, SshStore}, infra::repository::{DBCreateResutl, SqliteStore}},
};

use parser::{
  group::{GroupActions, GroupCommand},
  server::ServerCommmand
};



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


#[async_std::main]
async fn main() -> Result<(), ()> {
  env_logger::init();

  let config = match Config::exists() {
    Ok(exist) => {
      if exist {
        // Read configuration
        Config::read().expect("error")
      } else {
        let dto = prompt::config::ask().unwrap();
        Config::create(&dto).expect("error")
      }
    },
    Err(e) => {
      println!("Error {}", e);
      return Ok(())
    }
  };

  let pass = match prompt::password::ask(config.db_name.exists()) {
    Ok(p) => p,
    Err(e) => {
      println!("{}", e);
      return Ok(())
    }
  };

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

  if let Some (_command) = cli.command {
    todo!();
    // match command {
    //   Command::Group(gc) => {
    //     match gc.command {
    //       GroupActions::List => {
    //         match list_groups(&sqlite_repo, None).await {
    //           Ok(groups) => {
  
    //           },
    //           Err(e) => {
    //             panic!("{}", e);
    //           }
    //         };
    //       },
    //       GroupActions::Create(args) => {
  
    //       },
    //       GroupActions::Edit => {
    
    //       }
    //     }
    //   },
    //   Command::Server(cs) => {
  
    //   }
    // };

  } else {
    // Interactive mode
    let mut current_group: Option<Group> = None;

    loop {
      match app::group::list(&sqlite_repo, &current_group).await {
        Ok(groups) => {
          // Set options
          let mut options = prompt::group::transform::groups_as_vec(&groups);
          options.push("-------------------------------------".grey());
          if let None = current_group {
            for op in prompt::group::options::ROOT_OPTS { options.push(op.to_string()); }
          } else {
            for op in prompt::group::options::OPTS { options.push(op.to_string()); }
          }

          // Ask for option
          let str_opt = match Select::new("", options).prompt() {
            Ok(o) => o,
            Err(_) => {
              print_farewell();
              return Ok(())
            }
          };
          // Evluate selected option          
          if let Ok(predefined_option) = prompt::group::options::ExtraOptions::from_str(&str_opt) {
            match predefined_option {
              prompt::group::options::ExtraOptions::PreviuosGroup => {
                if let Some(g) = &current_group {
                  if let Some(parent) = g.parent_id {
                    if let Ok(prev) = app::group::get(&sqlite_repo, parent).await {
                      if let Some(previous_group) = prev {
                        current_group = Some(previous_group)
                      }
                    }
                  } else {
                    current_group = None
                  }
                }
              },
              prompt::group::options::ExtraOptions::CreateGroup => {
                let name = match prompt::group::ask_group() {
                  Ok(n) => n,
                  Err(_) => {
                    continue;
                  }
                };
                app::group::create(&sqlite_repo, &name, &current_group).await.unwrap();
                let st = time::Duration::from_millis(100);
                sleep(st);
              },
              prompt::group::options::ExtraOptions::AddServer => {
                let server = match prompt::server::ask() {
                  Ok(sp) => {
                    CreaterServer {
                      name: sp.name,
                      group_id: current_group.clone().unwrap().id,
                      host: sp.host,
                      port: sp.port,
                      user: sp.user
                    }
                  },
                  Err(_) => {
                    continue;
                  }
                };
                app::server::create(&sqlite_repo, server, &config.ssh_path.clone(), &pass).await.unwrap();
              },
              _ => {}
            };
          } else {
            // If select a group, enter o that group
            if let Some(group) = groups.iter().find(|g| str_opt == prompt::group::transform::group_as_str(g)) {
              current_group = Some(group.to_owned());
            }
            
          }
        },
        Err(e) => {
          panic!("{}", e);
        }
      }; 
    }
  }

}


fn print_farewell() {
  let mesasage = format!("Thanks for using {} a software created with {} by {}", "SSHY".yellow(), "♥".yellow(), "evesan".yellow().italic());

  println!("{}", mesasage);
}