use clap::Args;

#[derive(Args)]
pub struct AniagotujeArgs {
    /// Categories you want to steal from
    #[arg(required = true)]
    pub categories: Vec<String>
}

pub async fn steal(args: AniagotujeArgs, filename: Option<String>) {
    println!("categories: {:#?}", args.categories);
    println!("Steal ania gotuje");
}