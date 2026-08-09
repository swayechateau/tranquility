use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::vps;
use std::path::PathBuf;

pub async fn add(
    ctx: &Context,
    name: String,
    host: String,
    user: Option<String>,
    port: Option<u16>,
) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::add(
        Some(name),
        Some(host),
        user,
        port,
        None,
        None,
        &vps_file_path,
    )
    .await
    .map_err(|e| e.into())
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
        return Ok(());
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

    vps::connect(id_or_name, &vps_file_path)
        .await
        .map_err(|e| e.into())
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
    )
    .await
    .map_err(|e| e.into())
}

pub async fn delete(ctx: &Context, id_or_name: Option<String>) -> CoreResult<()> {
    let vps_file_path = ctx
        .config()
        .ok_or_else(|| crate::engine::Error::from_code(crate::engine::ErrorCode::ConfigNotFound))?
        .base
        .vps_file
        .clone();

    vps::delete(id_or_name, &vps_file_path)
        .await
        .map_err(|e| e.into())
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

    vps::script(id_or_name, inline_script, script_file, &vps_file_path)
        .await
        .map_err(|e| e.into())
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

    vps::copy(id_or_name, source, destination, dir, &vps_file_path)
        .await
        .map_err(|e| e.into())
}

pub async fn schema(ctx: &Context) -> CoreResult<()> {
    vps::schema(ctx).await.map_err(|e| e.into())
}
