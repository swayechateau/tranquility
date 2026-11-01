pub mod args;
pub mod cmd;

use std::process::ExitCode;
pub type CliResult<T> = Result<T, Error>;

use crate::{
    core::{bootstrap::build_app_context, exit_code, report_error},
    engine::{Error, ErrorCode},
};
use clap::Parser;
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt};

pub fn run() -> ExitCode {
    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime.block_on(async_run()),
        Err(err) => {
            let err = ErrorCode::ProcessFailure
                .error()
                .with_context_str("runtime", err);

            report_error(&err);
            exit_code(&err)
        }
    }
}

async fn async_run() -> ExitCode {
    let cli = args::Cli::parse();

    let default_level = match (cli.global.verbose, cli.global.quiet) {
        (_, q) if q >= 2 => Level::ERROR,
        (_, 1) => Level::WARN,
        (1, _) => Level::INFO,
        (2, _) => Level::DEBUG,
        (v, _) if v >= 3 => Level::TRACE,
        _ => Level::WARN,
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level.as_str()));
    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    if let Err(err) = validate_global_paths(&cli.global) {
        report_error(&err);
        return exit_code(&err);
    }

    let ctx = match build_app_context(cli.global.into()) {
        Ok(ctx) => ctx,
        Err(err) => {
            let err = ErrorCode::ProcessFailure
                .error()
                .with_context_str("app_context", err);
            report_error(&err);
            return exit_code(&err);
        }
    };

    let result = match cli.command {
        Some(args::Command::Info) => cmd::info::run(&ctx).await,
        Some(args::Command::App(args)) => cmd::app::run(&ctx, args).await,
        Some(args::Command::Font(args)) => cmd::font::run(&ctx, args).await,
        Some(args::Command::Logs(args)) => cmd::logs::run(&ctx, args).await,
        Some(args::Command::List(args)) => cmd::list::run(&ctx, args).await,
        Some(args::Command::Config(args)) => cmd::config::run(&ctx, args).await,
        Some(args::Command::Doctor(args)) => cmd::doctor::run(&ctx, args).await,
        Some(args::Command::Vps(args)) => cmd::vps::run(&ctx, args).await,
        None => cmd::info::run(&ctx).await,
    };

    if let Err(err) = result {
        report_error(&err);
        return exit_code(&err);
    }

    ExitCode::SUCCESS
}

/// Fail early when `-C/--cwd` or `--config` point at something that does not exist.
fn validate_global_paths(global: &args::GlobalArgs) -> Result<(), Error> {
    if !global.cwd.is_dir() {
        return Err(ErrorCode::IoFailure
            .error()
            .with_context("path", global.cwd.display().to_string())
            .with_context("error", "--cwd is not an existing directory"));
    }
    if let Some(config) = &global.config
        && !config.is_file()
    {
        return Err(ErrorCode::ConfigNotFound
            .error()
            .with_context("path", config.display().to_string())
            .with_context("error", "--config is not an existing file"));
    }
    Ok(())
}
