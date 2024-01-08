// ligthyear section header
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use bevy::core_pipeline::clear_color::ClearColorConfig;
// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::view::RenderLayers;
use bevy_egui::EguiPlugin;
// gltf
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::pbr::DirectionalLightShadowMap;
use serde::de;
use std::f32::consts::*;
use bevy::render::camera::OrthographicProjection;
use bevy::render::camera::ScalingMode;
use bevy::render::camera::Viewport;
// end of gltf

mod gui;
use client::KeyStates;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

mod user;
use user::info;

mod level;

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


// hardcoded, to simplify the code
pub const GAME_LEVEL: u8 = 1; // 1, 2, 3

/// it is a hybrid client or server
#[tokio::main]
async fn main() {
  // let (ip, the_port) = info::get_server_address();
  // let name = info::get_creature_name();
  // let level = info::mutate_to_level(&name); // later add to the server someway
  
  // dev gap
  let ip = Ipv4Addr::from_str(&"127.0.0.1").unwrap();
  let the_port = 8000;
  let name = "client";
  // end of dev gap
  
  let id = info::mutate_to_id(&name);
  
  let mut cli = Cli::parse();
  
  // As audit questions require, set ip:port, from the user terminal input steps.
  // It is not my initiative. Flags using at least looks easy for scripting.
  match &mut cli {
    Cli::Server { port  , .. } => {
      *port = the_port;
      println!("Server is running on {}:{}...", ip, port);
      println!("Game level is {}", GAME_LEVEL);
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
  
  // INJECTION fps on screen etc. Later can move to setup(), when it will be more clear
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
  // app.insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)));
  app.add_systems(
    Update,
    (
      show_fps_ui,
      change_window_title,
    )
  );
  app.insert_resource(KeyStates::default() );
  
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
      app.add_plugins(client_plugin);
      // app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
      if inspector {
        app.add_plugins(WorldInspectorPlugin::new());
      }
      
      // try to inject 3d model stuff
      app.insert_resource(DirectionalLightShadowMap { size: 4096 });
      app.add_systems(Startup, startup_setup);
      // end of 3d model stuff
      
    }
  }
}

fn startup_setup(
  mut commands: Commands,
  mut gizmo_config: ResMut<GizmoConfig>,
  asset_server: Res<AssetServer>
) {

  // make lines fat
  gizmo_config.line_width = 3.0;

  // --------------------------------------------
  // global camera. Must display the whole level.
  // Must be in front of the player cell.
  // So, it should be controllable by the keyboard.
  // As result the player is not visible.
  // --------------------------------------------
  commands.spawn((
    Camera3dBundle {
      camera: Camera {
        order: 0,
        viewport: Some(Viewport {
          physical_position: UVec2::new(0, 0),
          physical_size: UVec2::new(1000, 1000),
          depth: 0.0..1.0,
        }),
        ..default()
      },
      transform: Transform::from_xyz(2.0, 2.0, 1.0)
      .looking_at(Vec3::new(2.0, 22.0, 0.0), Vec3::Z),
      ..default()
    },

    /*
    EnvironmentMapLight {
      diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
      specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    },
    */
    
  ));
  
  
  // --------------------------------------------
  // minimap camera . Must display level and player position.
  // Other players are hidden. Orthographic projection from top.
  // --------------------------------------------
  commands.spawn((
    Camera3dBundle {
      camera_3d: Camera3d {
        clear_color: ClearColorConfig::None,
        ..default()
      },
      camera: Camera {
        viewport: Some(Viewport {
          physical_position: UVec2::new(0, 50), // y 750 to bottom
          physical_size: UVec2::new(250, 250),
          depth: 0.0..1.0,
        }),
        order: 1,
        ..default()
      },
      projection: OrthographicProjection {
        viewport_origin: Vec2::new(0.0, 0.0), // Top left corner of the screen
        // scale: 25.0 / 250.0, // Scale factor from level size to viewport size
        scaling_mode: ScalingMode::Fixed{width: 25.0, height: 25.0}, // Set the height of the camera view in world units
        ..default()
      }.into(),
      transform: Transform::from_xyz(-25.5, -0.5, 2.0)
      .looking_at(Vec3::new(-25.5, -0.5, 0.0), Vec3::Y),
      ..default()
    },
    UiCameraConfig { show_ui: false, },
  ));

  
  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      shadows_enabled: true,
      ..default()
    },
    cascade_shadow_config: CascadeShadowConfigBuilder {
      num_cascades: 1,
      maximum_distance: 1.6,
      ..default()
    }
    .into(),
    ..default()
  });
  
  // perspective world
  commands.spawn(SceneBundle {
    scene: asset_server.load("level1.gltf#Scene0"),
    ..default()
  });

  // minimap
  commands.spawn(SceneBundle {
    scene: asset_server.load("level1.gltf#Scene0"),
    transform: Transform::from_translation(Vec3::new(-25.0, 0.0, 0.0)),
    ..Default::default()
 });
  
  
}