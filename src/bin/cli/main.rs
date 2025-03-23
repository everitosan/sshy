mod prompt;
mod parser;

use std::{str::FromStr, thread::sleep, time};

use colorize::AnsiColor;
use inquire::Select;
use log::debug;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use clap::{Parser, Subcommand};

use sshy::{
  config::Config,
  ssh::{app::{self, credentials::AppCredentialsDto, server::CreateServerDto}, domain::{group::Group, SshStore}, infra::repository::{DBCreateResutl, SqliteStore}},
};

use parser::{
  group::GroupCommand,
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
    Err(_e) => {
      print_farewell();
      return Ok(())
    }
  };

  let options = SqliteConnectOptions::from_str(&config.db_name.to_str().unwrap()).unwrap()
    .pragma("key", pass.clone()) // Establece la clave de cifrado
    .create_if_missing(true) // Crea la BD si no existe
    .to_owned();

  let pool = SqlitePool::connect_with(options).await.unwrap(); 

  // DB Instance
  let db_res = match SqliteStore::try_create(&config.db_name, &pass).await {
    Ok(d) => d,
    Err(e) => {
      println!("{}", e);
      return Ok(());
    }
  };
  
  
  // let options = SqliteConnectOptions::from_str(&config.db_name.to_str().unwrap()).unwrap()
  //   .pragma("key", pass.clone()) // Establece la clave de cifrado
  //   .create_if_missing(true) // Crea la BD si no existe
  //   .to_owned();
  
  // let pool = SqlitePool::connect_with(options).await.unwrap(); 
  
  // let pool = SqlitePool::connect(&config.db_name.to_str().unwrap()).await.unwrap();
  let sqlite_repo = SqliteStore::new(&pool);
  sqlite_repo.initialize().await.unwrap();

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
          
          if let Some(cg) = &current_group {
            options.extend(prompt::server::transform::server_as_vec(&cg.servers));
            if options.len() > 0 {
              options.push("-------------------------------------".grey());
            }
            for op in prompt::group::options::OPTS { options.push(op.to_string()); }
          } else {
            if options.len() > 0 {
              options.push("-------------------------------------".grey());
            }
            for op in prompt::group::options::ROOT_OPTS { options.push(op.to_string()); }
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
                    CreateServerDto {
                      name: sp.name,
                      group_id: current_group.clone().unwrap().id,
                      host: sp.host,
                      port: sp.port,
                    }
                  },
                  Err(_) => {
                    continue;
                  }
                };
                match app::server::create(&sqlite_repo, server).await {
                  Ok(s) => {
                    // update current group
                    println!("{} server created", s.name);
                    let mut updated_group = current_group.clone().unwrap();
                    updated_group.servers.push(s);
                    current_group = Some(updated_group);
                  },
                  Err(e) => {
                    println!("error: {}", e);
                  }
                };
              },
              _ => {}
            };
          } else {
            // If select a group, enter to that group
            if let Some(group) = groups.iter().find(|g| str_opt == prompt::group::transform::group_as_str(g)) {
              current_group = Some(group.to_owned());
            }
            // if select a server, show options server
            if let Some(server) = current_group.clone().unwrap().servers.iter().find(|s| str_opt == prompt::server::transform::sever_as_str(s)) {
              println!("\t{}:{}", server.hostname.clone().magenta(), format!("{}", server.port).magenta());
              // Ask for option
              match Select::new("", prompt::server::options::OPTS.to_vec()).prompt() {
                Ok(predefined_server_option) =>  {
                  match predefined_server_option {
                    prompt::server::options::ExtraOptions::Connect => {
                      match app::credentials::get_for_server_id(&sqlite_repo, server.id).await {
                        Ok(credentials) => {
                          let selected_credential;
                          if credentials.len() == 0 { 
                            // Create credentials
                            let prompt_dto = prompt::credentials::ask().unwrap();
                            let app_dto = AppCredentialsDto {
                              name: prompt_dto.name,
                              private_key: prompt_dto.private_key,
                              public_key: prompt_dto.public_key,
                              user: prompt_dto.user,
                              server_id: server.id
                            };
                            if let Ok(res) = app::credentials::create_for_server(&sqlite_repo, &server, &app_dto, &config.ssh_path, &pass).await {
                              selected_credential = res;
                            } else {
                              continue;
                            }
                          } else {
                            let cred_options = prompt::credentials::transform::credentials_as_vec(&credentials);
                            let selected_credential_str = match Select::new("Select credentials to use:", cred_options).prompt() {
                              Ok(o) => o,
                              Err(_) => {
                                continue;
                              }
                            };
                            if let Some(s) = credentials.iter().find(|c| selected_credential_str == prompt::credentials::transform::credential_as_str(c)) {
                              selected_credential = s.clone();
                            } else {
                              continue;
                            }
                          }

                          app::server::connect(server, &selected_credential, &config.ssh_path.clone()).await.unwrap();
                        }, 
                        Err(e) => {
                          println!("Some error ocurred: {}", e);
                        }
                      };
                    },
                    prompt::server::options::ExtraOptions::EditServer => todo!(),
                    prompt::server::options::ExtraOptions::DeleteServer => todo!(),
                    prompt::server::options::ExtraOptions::Back => {},
                  }
                },
                Err(_) => {
                  continue;
                }
              };
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