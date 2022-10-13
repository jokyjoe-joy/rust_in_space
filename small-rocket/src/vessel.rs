use krpc_mars::RPCClient;
use crate::space_center;
use crate::space_center::Vessel;
use log::{ info };

/// <doc> 
/// <summary> Runs every experiment found on the vessel. </summary> 
/// </doc>
pub fn run_all_science(client: &RPCClient, vessel: &Vessel) -> Result<(), krpc_mars::error::Error> {
    info!("Running all science experiments...");
    let parts = client.mk_call(&vessel.get_parts())?;
    let experiments = client.mk_call(&parts.get_experiments())?;

    let mut cum_science_pts = 0.0;

    for experiment in experiments {
        client.mk_call(&experiment.run())?;
        // TODO: It doesn't show the real science value. 
        // Vec<ScienceData> is for some reason len() = 0.
        let data = &client.mk_call(&experiment.get_science_subject())?;
        cum_science_pts += client.mk_call(&data.get_scientific_value())?;
    }

    info!("Successfully collected: {} science value.", cum_science_pts);
    Ok(())
}

/// <doc> 
/// <summary> Deploys every parachute found on the vessel. </summary> 
/// </doc>
pub fn deploy_parachutes(client: &RPCClient, parts: &space_center::Parts) -> Result<(), krpc_mars::error::Error> {
    info!("Deploying parachutes...");
    let parachutes = client.mk_call(&parts.get_parachutes())?;
    
    let mut cum_deployed = 0;
    for parachute in parachutes {
        client.mk_call(&parachute.deploy())?;
        if client.mk_call(&parachute.get_deployed())? {
            cum_deployed += 1;
        }
    }
    
    info!("Deployed {} parachute(s).", cum_deployed);
    

    Ok(())
}