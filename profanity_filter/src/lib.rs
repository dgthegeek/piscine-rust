pub struct Message {
    content: String,
    user: String
}

  impl Message {
    pub fn new(ms: String, u: String) -> Message {
      Message {
        content: ms,
        user: u
      }
    }
    pub fn send_ms(&self) -> Option<&str> {
      if self.content.is_empty() || self.content.contains("stupid") {
              None
          } else {
              Some(self.content.as_str())
          }
    }
  }
  
  pub fn check_ms(ms: &Message) -> (bool, &str) {
     let result = ms.send_ms();
     match result {
      None => (false, "ERROR: illegal"),
      Some(content) => (true, content),
     }
  }