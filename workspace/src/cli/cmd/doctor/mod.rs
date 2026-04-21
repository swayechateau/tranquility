use crate::{
    cli::CliResult,
    core::{context::Context, doctor},
};
use clap::{Args as ClapArgs, Subcommand};

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "Inspect and diagnose the local Tranquility environment")]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<DoctorSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DoctorSubcommand {
    /// Attempt safe local repairs for issues detected by doctor
    Fix,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(DoctorSubcommand::Fix) => doctor::fix(ctx).await.map_err(|e| e.into()),
        None => doctor::run(ctx).await.map_err(|e| e.into()),
    }
}
