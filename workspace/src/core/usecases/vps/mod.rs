use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::vps;
use std::io::IsTerminal;
use std::path::PathBuf;

fn opts(ctx: &Context) -> vps::Opts {
    use crate::engine::models::runtime::mode::RunMode;
    vps::Opts {
        dry_run: ctx.dry_run(),
        interactive: ctx.is_interactive()
            && std::io::stdin().is_terminal()
            && std::io::stderr().is_terminal(),
        assume_yes: ctx.auto_yes() || matches!(ctx.runtime().mode(), RunMode::Ci),
        timeout: None,
    }
}

fn opts_with_timeout(ctx: &Context, timeout: Option<u32>) -> vps::Opts {
    vps::Opts {
        timeout,
        ..opts(ctx)
    }
}

fn vps_file_path(ctx: &Context) -> CoreResult<PathBuf> {
    Ok(ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone())
}

#[allow(clippy::too_many_arguments)]
pub async fn add(
    ctx: &Context,
    id: Option<String>,
    name: Option<String>,
    host: String,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<PathBuf>,
    post_connect_script: Option<String>,
    jump_host: Option<String>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;

    let name = name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| host.clone());

    vps::add(
        id,
        name,
        host,
        user,
        port,
        private_key,
        post_connect_script,
        jump_host,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn list(ctx: &Context) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;

    let rows = vps::vps_list_rows(&vps_file_path, None, None)?;

    if rows.is_empty() {
        ctx.ui()
            .logger()
            .warn("No VPS profiles configured. Run 'tquil vps add' to add one.");
        // Structured formats still emit an (empty) table so output stays parseable.
        if !ctx.ui().is_structured() {
            return Ok(());
        }
    }

    let table = ctx.ui().table(
        vec![
            "Id".into(),
            "Name".into(),
            "User".into(),
            "Host".into(),
            "Port".into(),
        ],
        rows.into_iter()
            .map(|r| vec![r.id, r.name, r.user, r.host, r.port])
            .collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}

pub async fn connect(
    ctx: &Context,
    id_or_name: Option<String>,
    timeout: Option<u32>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;
    vps::connect(id_or_name, &vps_file_path, &opts_with_timeout(ctx, timeout)).await
}

#[allow(clippy::too_many_arguments)]
pub async fn update(
    ctx: &Context,
    id_or_name: Option<String>,
    new_id: Option<String>,
    name: Option<String>,
    host: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<String>,
    post_connect_script: Option<String>,
    jump_host: Option<String>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;

    vps::update(
        id_or_name,
        new_id,
        name,
        host,
        user,
        port,
        private_key,
        post_connect_script,
        jump_host,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn delete(ctx: &Context, id_or_name: Option<String>) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;
    vps::delete(id_or_name, &vps_file_path, &opts(ctx)).await
}

#[allow(clippy::too_many_arguments)]
pub async fn script(
    ctx: &Context,
    id_or_name: Option<String>,
    inline_script: Option<String>,
    script_file: Option<PathBuf>,
    all: bool,
    timeout: Option<u32>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;

    vps::script(
        id_or_name,
        inline_script,
        script_file,
        all,
        &vps_file_path,
        &opts_with_timeout(ctx, timeout),
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn copy(
    ctx: &Context,
    id_or_name: Option<String>,
    to_id: Option<String>,
    source: String,
    destination: String,
    direction: Option<String>,
    all: bool,
    timeout: Option<u32>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;

    let dir = match direction.as_deref() {
        Some("pull") => "pull",
        Some("remote") => "remote",
        _ => "push",
    };

    vps::copy(
        id_or_name,
        to_id,
        source,
        destination,
        dir,
        all,
        &vps_file_path,
        &opts_with_timeout(ctx, timeout),
    )
    .await
}

pub async fn ping(
    ctx: &Context,
    id_or_name: Option<String>,
    all: bool,
    timeout: Option<u32>,
) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;
    let results = vps::ping(
        id_or_name,
        all,
        &vps_file_path,
        &opts_with_timeout(ctx, timeout.or(Some(5))),
    )
    .await?;

    if results.iter().any(|(_, ok)| !ok) {
        let unreachable: Vec<&str> = results
            .iter()
            .filter(|(_, ok)| !ok)
            .map(|(id, _)| id.as_str())
            .collect();
        return Err(
            crate::engine::Error::from_code(crate::engine::ErrorCode::ProcessNonZeroExit)
                .with_context("unreachable", unreachable.join(", ")),
        );
    }
    Ok(())
}

pub async fn export(ctx: &Context, output: Option<PathBuf>) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;
    let content = vps::export_entries(&vps_file_path)?;

    match output {
        Some(path) => {
            std::fs::write(&path, &content).map_err(|e| {
                crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
                    .with_context("path", path.display().to_string())
                    .with_context("error", e.to_string())
            })?;
            println!("✅ Exported VPS profiles to {}", path.display());
        }
        None => print!("{content}"),
    }
    Ok(())
}

pub async fn import(ctx: &Context, file: PathBuf, merge: bool) -> CoreResult<()> {
    let vps_file_path = vps_file_path(ctx)?;
    let (imported, skipped) = vps::import_entries(&file, &vps_file_path, merge, &opts(ctx))?;
    if !ctx.dry_run() {
        println!("✅ Imported {imported} profile(s) ({skipped} skipped as duplicates).");
    }
    Ok(())
}

pub async fn schema(ctx: &Context) -> CoreResult<()> {
    vps::schema(ctx).await
}
