
use std::{io::{self, Write}, net::Ipv4Addr};

/// For request the server ip:port from user terminal input,
/// and store it in the variable.
/// 
/// Only numbers , dots and colon allowed,
/// otherwise print error and loop again to collect the ip:port.
pub fn get_server_address() -> (Ipv4Addr, u16) {
  let mut input = String::new();
  loop {
    println!(
      "===================================\n=  Enter server \"IPv4:port\" pair  =\n= (numbers, dots, and colon only) =\n==================================="
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.chars().all(|c| c.is_digit(10) || c == '.' || c == ':') {
      let mut parts = input.splitn(2,':');
      
      // try to parse the IPv4 address
      let raw_ip = match parts.next(){
        Some(raw_ip) => raw_ip,
        None => {
          println!("Invalid input. Please use only numbers, dots.\nThe IPv4 address must be in the format [u8; 4]: x.x.x.x");
          input.clear();
          continue;
        }
      };
      let ip = match raw_ip.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(e) => {
          println!("Invalid input. Fail to parse ip string into IPv4\nError: {}", e);
          input.clear();
          continue;
        }
      };
      
      // try to parse the port
      let raw_port = match parts.next(){
        Some(raw_port) => raw_port,
        None => {
          println!("Invalid input. Please use only numbers.\nThe port must be in the range [0; 65535]");
          input.clear();
          continue;
        }
      };
      let port = match raw_port.parse::<u16>() {
        Ok(port) => port,
        Err(e) => {
          println!("Invalid input. Fail to parse port string into u16\nError: {}", e);
          input.clear();
          continue;
        }
      };
      
      return (ip, port);
    } else {
      println!("Invalid input. Please use only numbers, dots, and colon.");
      input.clear();
    }
  }
}

/// Request the name from user terminal input,
/// and store it in the variable.
/// 
/// Only the english letters and the numbers are allowed,
/// otherwise print error message and loop again to collect the name.
pub fn get_creature_name() -> String {
  let mut input = String::new();
  loop {
    println!(
      "====================================== \n=      Enter unique short name       = \n= (English letters and numbers only) = \n====================================== \n= The name used to create client id. = \n= If connection is timed out         = \n= and ip:port pair is correct        = \n= then try to change the name.       = \n======================================"
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.chars().all(|c| c.is_ascii_alphanumeric()) {
      // limit the name length to 20 characters
      if input.len() > 20 {
        println!("Ok, Joker. Use the number of characters\nwhich is equals to the diameter of your friend\nin centimeters. And let it be 20cm or less, please 0:).");
        input.clear();
        continue;
      }
      
      return input.to_string();
    } else {
      println!("Invalid input. Please use only English letters and numbers.");
      input.clear()
    }
  }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn mutate_to_id(name: &str) -> u16 {
  let mut hasher = DefaultHasher::new();
  name.hash(&mut hasher);
  let hash_value = hasher.finish();
  let id = hash_value as u16;
  id
}

/// get the level number from the port last digit "2" -> 2 , "3" -> 3 , otherwise -> 1
/// 
/// used to determine the level map to use.
/// 
/// Client will load the gltf according this level number.
/// 
/// Server will use level 2d map according this level number, to check which cells are free to move.
pub fn mutate_to_level(port: &str) -> u8 {
  let level;
  if port.ends_with("3") { level = 3; }
  else if port.ends_with("2") { level = 2; }
  else { level = 1; }
  level
}
