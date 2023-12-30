// ligthyear section header
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;

mod gui;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

mod keyboard;
use keyboard::activity::listen_keys;

mod user;
use user::info;

mod connection;
use connection::connect;

//lightyear section
// ! Run with
// ! - `cargo run --example simple_box -- server`
// ! - `cargo run --example simple_box -- client -c 1`
mod client;
mod protocol;
mod server;
mod shared;

use std::net::Ipv4Addr;
use std::str::FromStr;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::client::MyClientPlugin;
use crate::server::MyServerPlugin;
use lightyear::netcode::{ClientId, Key};
use lightyear::prelude::TransportConfig;
/* 
/// it is a hybrid client or server
fn main() {
  let server_address = info::get_server_address();
  let name = info::get_user_name();
  if let Err(error) = connect::connect_to(&server_address, &name) {
    println!("Connection error: {}", error);
    return;
  }
  
  // if connection is successful, then start the game. Very raw
  
  App::new()
  .add_plugins(
    (
      DefaultPlugins.set(
        WindowPlugin {
          primary_window: Some(
            Window {
              resolution: (1000., 1000.).into(),
              title: "Thank you for your help! I want to complete it before February".into(),
              ..default()
            }
          ),
          ..default()
        }
      ),
      EguiPlugin,
      // LogDiagnosticsPlugin::default(), // to print fps in terminal
      FrameTimeDiagnosticsPlugin::default(),
    )
    
  )
  // .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 0.0))) // it is dead/black
  
  .add_systems(
    Update,
    (
      show_fps_ui,
      change_window_title,
      listen_keys,
    )
  )
  .run();
  
  // placeholder for game over. Probably just close the window and/or exit. Not sure how make it the best way
}
*/

#[tokio::main]
async fn main() {
  let (ip, the_port) = info::get_server_address();
  let name = info::get_creature_name();
  let id = info::mutate_to_id(&name);
  
  let mut cli = Cli::parse();

  // As audit questions require, set ip:port, from the user terminal input steps.
  // It is not my initiative. Flags using at least looks easy for scripting.
  match &mut cli {
    Cli::Server { port  , .. } => {
      *port = the_port;
      println!("Server is running on {}:{}...", ip, port);
    },
    Cli::Client {client_id, server_addr, server_port, .. } => {
      *client_id = id;
      *server_addr = ip;
      *server_port = the_port;
      println!(
        "Client \"{}\" with id \"{}\" is trying to call {}:{}...",
        name, id,
        server_addr, server_port
      );
    },
    _ => {},
  }

  let mut app = App::new();
  
  // fps on screen etc. Later can move to setup(), when it will be more clear
  app.add_plugins((
    DefaultPlugins.set(
      WindowPlugin {
        primary_window: Some(
          Window {
            resolution: (1000., 1000.).into(),
            title: "Thank you for your help! I want to complete it before February".into(),
            ..default()
          }
        ),
        ..default()
      }
    ),
    EguiPlugin,
      // LogDiagnosticsPlugin::default(), // to print fps in terminal
      FrameTimeDiagnosticsPlugin::default(),
  ));
  app.add_systems(
    Update,
    (
      show_fps_ui,
      change_window_title,
      // listen_keys,
    )
  );

  setup(&mut app, cli);
  
  app.run();
}

// Use a port of 0 to automatically select a port
pub const CLIENT_PORT: u16 = 0;
pub const SERVER_PORT: u16 = 5000;
pub const PROTOCOL_ID: u64 = 0;

pub const KEY: Key = [0; 32];

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Transports {
  Udp,
  Webtransport,
}

#[derive(Parser, PartialEq, Debug)]
enum Cli {
  SinglePlayer,
  Server {
    #[arg(long, default_value = "false")]
    headless: bool,
    
    #[arg(short, long, default_value = "false")]
    inspector: bool,
    
    #[arg(short, long, default_value_t = SERVER_PORT)]
    port: u16,
    
    #[arg(short, long, value_enum, default_value_t = Transports::Udp)]
    transport: Transports,
  },
  Client {
    #[arg(short, long, default_value = "false")]
    inspector: bool,
    
    #[arg(short, long, default_value_t = 0)]
    client_id: u16,
    
    #[arg(long, default_value_t = CLIENT_PORT)]
    client_port: u16,
    
    #[arg(long, default_value_t = Ipv4Addr::LOCALHOST)]
    server_addr: Ipv4Addr,
    
    #[arg(short, long, default_value_t = SERVER_PORT)]
    server_port: u16,
    
    #[arg(short, long, value_enum, default_value_t = Transports::Udp)]
    transport: Transports,
  },
}

fn setup(app: &mut App, cli: Cli) {
  match cli {
    Cli::SinglePlayer => {}
    Cli::Server {
      headless,
      inspector,
      port,
      transport,
    } => {
      let server_plugin = MyServerPlugin {
        headless,
        port,
        transport,
      };
      if !headless {
        // app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
      } else {
        app.add_plugins(MinimalPlugins);
      }
      if inspector {
        app.add_plugins(WorldInspectorPlugin::new());
      }
      app.add_plugins(server_plugin);
    }
    Cli::Client {
      inspector,
      client_id,
      client_port,
      server_addr,
      server_port,
      transport,
    } => {
      let client_plugin = MyClientPlugin {
        client_id: client_id as ClientId,
        client_port,
        server_addr,
        server_port,
        transport,
      };
      // app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
      if inspector {
        app.add_plugins(WorldInspectorPlugin::new());
      }
      app.add_plugins(client_plugin);
    }
  }
}