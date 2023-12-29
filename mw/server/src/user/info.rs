
use std::io::{self, Write};

  /// For request the server ip:port from user terminal input,
  /// and store it in the variable.
  /// 
  /// Only numbers , dots and colon allowed,
  /// otherwise print error and loop again to collect the ip:port.
  pub fn get_server_address() -> String {
      let mut input = String::new();
      loop {
          print!("Enter server address (numbers, dots, and colon only): ");
          io::stdout().flush().unwrap();
          io::stdin().read_line(&mut input).unwrap();
          input = input.trim().to_string();
          if input.chars().all(|c| c.is_digit(10) || c == '.' || c == ':') {
              return input.to_string();
          } else {
              println!("Invalid input. Please use only numbers, dots, and colon.");
              input.clear();
          }
      }
  }

  /// Request the user name from user terminal input,
  /// and store it in the variable.
  /// 
  /// Only the english letters and the numbers are allowed,
  /// otherwise print error message and loop again to collect the name.
  pub fn get_user_name() -> String {
      let mut input = String::new();
      loop {
          print!("Enter your name (English letters and numbers only): ");
          io::stdout().flush().unwrap();
          io::stdin().read_line(&mut input).unwrap();
          input = input.trim().to_string();
          if input.chars().all(|c| c.is_ascii_alphanumeric()) {
              return input.to_string();
          } else {
              println!("Invalid input. Please use only English letters and numbers.");
              input.clear()
          }
      }
  }