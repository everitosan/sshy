use clap::{Parser, Subcommand};

/*
* 🅶🆁🅾🆄🅿
*/
#[derive(Debug, Parser)]
pub struct GroupCommand {
  #[command(subcommand)]
  pub command: GroupActions
}

#[derive(Debug, Subcommand)] 
pub enum GroupActions {
  /// List all registered
  List,
  /// Create a new group
  Create(CreateOpts),
  /// Edit an existing group
  Edit
}

#[derive(Debug, Parser)]
pub struct CreateOpts{
  /// Nombre del grupo
  #[arg(short, long)]
  name: Option<String>
}
