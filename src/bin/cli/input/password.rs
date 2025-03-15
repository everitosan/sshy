use inquire::Password;


pub fn ask(exists: bool) -> Option<String> {
  if exists {
    let p = match Password::new("Type secret password").without_confirmation().prompt() {
      Ok(p) => p,
      Err(_) => {return None}
    };
    return Some(p)
  }
  let p = match Password::new("Type secret password").prompt() {
    Ok(p) => p,
    Err(_) => { return None}
  };
  Some(p)
}