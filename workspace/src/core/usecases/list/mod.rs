use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::{apps, font, vps};
use crate::engine::models::category::Category;

pub async fn apps(ctx: &Context, server: bool, categories: Vec<Category>) -> CoreResult<()> {
    let apps_file = ctx.config().map(|c| c.base.applications_file.as_path());
    let rows = apps::app_list_rows(server, categories, apps_file);

    if rows.is_empty() {
        ctx.ui()
            .logger()
            .warn("No applications found matching the given filters.");
        return Ok(());
    }

    let table = ctx.ui().table(
        vec![
            "Name".into(),
            "Categories".into(),
            "Server".into(),
            "Installed".into(),
        ],
        rows.into_iter()
            .map(|r| {
                vec![
                    r.name,
                    r.categories,
                    r.server.to_string(),
                    r.installed.to_string(),
                ]
            })
            .collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}

pub async fn fonts(ctx: &Context, installed: bool, all: bool) -> CoreResult<()> {
    let rows = font::font_list_rows(installed, all);

    if rows.is_empty() {
        let msg = if installed {
            "No Nerd Fonts are currently installed."
        } else {
            "No fonts to display."
        };
        ctx.ui().logger().warn(msg);
        return Ok(());
    }

    let table = ctx.ui().table(
        vec!["Font".into(), "Status".into()],
        rows.into_iter()
            .map(|r| {
                vec![
                    r.name.to_string(),
                    if r.installed {
                        "installed"
                    } else {
                        "not installed"
                    }
                    .to_string(),
                ]
            })
            .collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}

pub async fn vps(ctx: &Context, user: Option<&str>, host: Option<&str>) -> CoreResult<()> {
    let vps_file = ctx
        .config()
        .map(|c| c.base.vps_file.clone())
        .unwrap_or_else(|| {
            crate::engine::config::BaseConfig::config_dir()
                .map(|d| d.join("vps.yaml"))
                .unwrap_or_default()
        });

    let rows = vps::vps_list_rows(&vps_file, user, host)?;

    if rows.is_empty() {
        let msg = if user.is_some() || host.is_some() {
            "No VPS profiles match the given filters."
        } else {
            "No VPS profiles configured. Run 'tquil vps add' to add one."
        };
        ctx.ui().logger().warn(msg);
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

pub async fn categories(ctx: &Context) -> CoreResult<()> {
    let names = apps::category_list_rows();
    let table = ctx.ui().table(
        vec!["Category".into()],
        names.into_iter().map(|n| vec![n]).collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}
