use clap::Args;
use log::info;
use crate::prelude::*;

#[derive(Args)]
pub struct CakeitArgs {
}

pub async fn steal(args: CakeitArgs) -> Vec<Recipe>{
    info!("Stealing from cakeit.pl ...");
    Vec::new()
}