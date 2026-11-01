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
    }
}

pub async fn add(
    ctx: &Context,
    name: Option<String>,
    host: String,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<PathBuf>,
) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    let name = name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| host.clone());

    vps::add(
        name,
        host,
        user,
        port,
        private_key,
        None,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn list(ctx: &Context) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

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

pub async fn connect(ctx: &Context, id_or_name: Option<String>) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::connect(id_or_name, &vps_file_path, &opts(ctx)).await
}

pub async fn update(
    ctx: &Context,
    id_or_name: String,
    name: Option<String>,
    host: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<String>,
) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::update(
        id_or_name,
        name,
        host,
        user,
        port,
        private_key,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn delete(ctx: &Context, id_or_name: Option<String>) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::delete(id_or_name, &vps_file_path, &opts(ctx)).await
}

pub async fn script(
    ctx: &Context,
    id_or_name: Option<String>,
    inline_script: Option<String>,
    script_file: Option<PathBuf>,
) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::script(
        id_or_name,
        inline_script,
        script_file,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn copy(
    ctx: &Context,
    id_or_name: Option<String>,
    source: String,
    destination: String,
    direction: Option<String>,
) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    let dir = match direction.as_deref() {
        Some("pull") => "pull",
        Some("remote") => "remote",
        _ => "push",
    };

    vps::copy(
        id_or_name,
        source,
        destination,
        dir,
        &vps_file_path,
        &opts(ctx),
    )
    .await
}

pub async fn schema(ctx: &Context) -> CoreResult<()> {
    vps::schema(ctx).await
}
