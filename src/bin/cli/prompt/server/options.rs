use strum_macros::{Display, EnumString};

// Select menu options
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum ExtraOptions {
  #[strum(to_string = "[.] Edit this server")]
  EditServer,
  #[strum(to_string = "[¤] Connect")]
  Connect,
  #[strum(to_string = "[-] Delete this server")]
  DeleteServer,
  #[strum(to_string = "[«] Back")]
  Back,
}


pub const OPTS: [&'static ExtraOptions; 4] = [
  &ExtraOptions::Back,
  &ExtraOptions::Connect,
  &ExtraOptions::EditServer,
  &ExtraOptions::DeleteServer,
];