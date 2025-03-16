use strum_macros::{Display, EnumString};

// Select menu options
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum ExtraOptions {
  #[strum(to_string = "« Back")]
  PreviuosGroup,
  #[strum(to_string = "[☉] Add server")]
  AddServer,
  #[strum(to_string = "[+] Create group")]
  CreateGroup,
  #[strum(to_string = "[.] Edit this group")]
  EditGroup,
  #[strum(to_string = "[-] Delete this group")]
  DeleteGroup,
}


pub const ROOT_OPTS: [&'static ExtraOptions; 1] = [&ExtraOptions::CreateGroup];
pub const OPTS: [&'static ExtraOptions; 5] = [
  &ExtraOptions::PreviuosGroup,
  &ExtraOptions::AddServer,
  &ExtraOptions::CreateGroup,
  &ExtraOptions::EditGroup,
  &ExtraOptions::DeleteGroup,
];