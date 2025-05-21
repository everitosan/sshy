use sshy::ssh::domain::credentials::Credentials;

pub fn credential_as_str(credentials: &Credentials) -> String {
  format!("• {} ({})", credentials.name, credentials.user)
}


pub fn credentials_as_vec(credentials: &Vec<Credentials>) -> Vec<String> {
  credentials.iter().map(|s| credential_as_str(s)).collect()
}
