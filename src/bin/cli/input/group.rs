use inquire::Text;
use sshy::{
  error::Result,
  ssh::domain::Group
};

pub fn group_as_str(group: &Group) -> String {
  format!("{}-{}", group.id, group.name)
}

pub fn groups_as_vec(groups: &Vec<Group>) -> Vec<String> {
  groups.iter().map(|g| group_as_str(g)).collect()
}


pub fn ask_group() -> Result<String> {
  let name = Text::new("Group name").prompt().unwrap();
  Ok(name)
}