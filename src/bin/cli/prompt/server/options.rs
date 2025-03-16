use strum_macros::{Display, EnumString};

// Select menu options
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum ExtraOptions {
  #[strum(to_string = "[.] Edit this server")]
  EditServer,
  #[strum(to_string = "[-] Delete this server")]
  DeleteServer,
  #[strum(to_string = "[«] Back")]
  Back,
}


pub const OPTS: [&'static ExtraOptions; 3] = [
  &ExtraOptions::Back,
  &ExtraOptions::EditServer,
  &ExtraOptions::DeleteServer,
];