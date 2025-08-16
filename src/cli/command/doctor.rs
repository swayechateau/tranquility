// Module: Command/Doctor
// Location: cli/src/command/doctor/mod.rs
use clap::{arg, Args};

use crate::core::tquil;

#[derive(Args, Debug)]
pub struct DoctorCommand {
    /// Fix any issues automatically
    #[arg(long)]
    fix: bool,
}

pub fn run(cmd: DoctorCommand, dry_run: bool) -> anyhow::Result<()> {
    tquil::doctor::run_doctor(cmd.fix, dry_run);
    Ok(())
}
