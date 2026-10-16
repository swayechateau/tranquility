use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::apps;
use crate::engine::models::category::Category;

/// Install applications using the engine capability
pub async fn install(ctx: &Context, all: bool, server: bool) -> CoreResult<()> {
    apps::install_apps_command(all, server, ctx.dry_run()).await
}

/// Uninstall applications using the engine capability
pub async fn uninstall(ctx: &Context, all: bool, server: bool) -> CoreResult<()> {
    apps::uninstall_apps_command(all, server, ctx.dry_run()).await
}

/// List applications (optionally filtered)
pub async fn list(ctx: &Context, server: bool, categories: Vec<Category>) -> CoreResult<()> {
    let rows = apps::app_list_rows(server, categories, None);

    if rows.is_empty() {
        ctx.ui().logger().warn("No applications to display.");
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

/// List all application categories
pub async fn categories(ctx: &Context) -> CoreResult<()> {
    let rows = apps::category_list_rows();

    if rows.is_empty() {
        ctx.ui().logger().warn("No categories to display.");
        return Ok(());
    }

    let table = ctx.ui().table(
        vec!["Category".into()],
        rows.into_iter().map(|category| vec![category]).collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}
