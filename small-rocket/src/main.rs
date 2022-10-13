extern crate rust_in_space;
use krpc_mars::RPCClient;
use rust_in_space::*;
use space_center::{ Control };
use std::{thread, time};
use std::io::Write;
use chrono::Local;
extern crate krpc_mars;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

/// <doc> 
/// <summary> Checks and sets SAS to stability assist. </summary>
/// </doc>
fn do_preflight_preps(client: &RPCClient, control: &Control) -> Result<(), krpc_mars::error::Error> {
    info!("Doing preflight checks...");
    // SAS
    client.mk_call(&control.set_sas(true))?;
    client.mk_call(&control.set_sas_mode(space_center::SASMode::StabilityAssist))?;

    info!("Preflight checks done.");

    Ok(())
}

fn main() -> Result<(), krpc_mars::error::Error> {
    // LOGGING
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )})
        .filter(None, log::LevelFilter::Info)
        .init();

    // Declaring variables for easier and shorter use later.
    let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
    let vessel = client.mk_call(&space_center::get_active_vessel())?;
    let control = client.mk_call(&space_center::Vessel::get_control(&vessel))?;
    let surface_ref_frame = client.mk_call(&vessel.get_surface_reference_frame())?;
    let surface_ref_flight = client.mk_call(&vessel.flight(&surface_ref_frame))?;
    let parts = client.mk_call(&vessel.get_parts())?;

    do_preflight_preps(&client, &control)?;

    // Initiating boosters
    info!("Commencing launch.");
    client.mk_call(&space_center::Control::activate_next_stage(&control))?;

    let mut altitude = client.mk_call(&surface_ref_flight.get_surface_altitude())?;
    while altitude < 10000.0 {
        debug!("Current altitude: {}", altitude);
        thread::sleep(time::Duration::from_secs(1));
        altitude = client.mk_call(&surface_ref_flight.get_surface_altitude())?;
    }
    
    vessel::run_all_science(&client, &vessel)?;
    
    info!("Decoupling boosters...");
    client.mk_call(&space_center::Control::activate_next_stage(&control))?;
    // TODO: Check if successfully decoupled.

    info!("Orientating towards retrograde...");
    client.mk_call(&control.set_sas_mode(space_center::SASMode::Retrograde))?;


    let mut altitude = client.mk_call(&surface_ref_flight.get_surface_altitude())?;
    while altitude > 3000.0 {
        debug!("Current altitude: {}", altitude);
        thread::sleep(time::Duration::from_secs(1));
        altitude = client.mk_call(&surface_ref_flight.get_surface_altitude())?;
    }

    vessel::deploy_parachutes(&client, &parts)?;

    Ok(())
}