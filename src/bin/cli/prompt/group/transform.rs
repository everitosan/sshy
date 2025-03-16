use sshy::ssh::domain::Group;


pub fn group_as_str(group: &Group) -> String {
  format!("☖ {}", group.name.clone())
}

pub fn groups_as_vec(groups: &Vec<Group>) -> Vec<String> {
  groups.iter().map(|g| group_as_str(g)).collect()
}
