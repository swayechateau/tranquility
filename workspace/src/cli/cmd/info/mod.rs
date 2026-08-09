use crate::{
    cli::CliResult,
    core::{context::Context, info},
};

pub async fn run(ctx: &Context) -> CliResult<()> {
    info::show(ctx).await.map_err(|e| e.into())
}
