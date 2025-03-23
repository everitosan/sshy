use sshy::ssh::domain::server::Server;

pub fn sever_as_str(server: &Server) -> String {
  format!("☍ {}", server.name)
}


pub fn server_as_vec(servers: &Vec<Server>) -> Vec<String> {
  servers.iter().map(|s| sever_as_str(s)).collect()
}
