use clap::Args;
use log::info;
use crate::prelude::*;

#[derive(Args)]
pub struct PosypaneArgs {
}

pub async fn steal(args: PosypaneArgs) -> Vec<Recipe>{
    info!("Stealing from posypane.pl ...");
    Vec::new()
}