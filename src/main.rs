extern crate krabmaga;
use krabmaga::*;

use crate::model::state::network_state::UrbanNetworkState;
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::bevy::prelude::FixedUpdate, krabmaga::visualization::fields::network::NetworkRender,
    krabmaga::visualization::visualization::Visualization,
};

mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;
///Initial infected nodes
pub static INITIAL_INFECTED_PROB: f64 = 0.01;
pub static INIT_EDGES: usize = 2;
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
pub static RECOVERY_CHANCE: f64 = 0.30;
pub static GAIN_RESISTANCE_CHANCE: f64 = 0.20;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() -> Result<(), std::io::Error> {
    use std::{
        env,
        ffi::OsStr,
        path::{self, Path},
    };

    let step: u64 = 110;
    let dim: (f32, f32) = (100., 100.);
    let num_nodes = 3_000;
    let num_agents = 5_000;
    // let urban_network =
    //     UrbanNetworkState::new(dim, num_agents, num_nodes, DISCRETIZATION, TOROIDAL);
    let mut osm_file_path = env::current_dir()?;
    osm_file_path.push("src/data/middlebury.osm.pbf");
    print!("{:?}", &osm_file_path);
    match UrbanNetworkState::from_osm_file(&osm_file_path, num_agents, DISCRETIZATION, TOROIDAL) {
        Ok(urban_network) => {
            simulate!(urban_network, step, 1, false);
        }

        Err(e) => {
            print!("{:?}", e);
        }
    }
    Ok(())
    //simulate!(urban_network, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let dim: (f32, f32) = (500., 500.);
    let num_nodes = 100;
    let epidemic_network = UrbanNetworkState::new(dim, num_nodes, DISCRETIZATION, TOROIDAL);

    let mut app = Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::rgb(255., 255., 255.))
        .setup::<VisState, UrbanNetworkState>(VisState, epidemic_network);
    app.add_systems(FixedUpdate, UrbanNetworkState::render);
    app.run();
}
