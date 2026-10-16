use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::font as font_caps;

pub async fn install(ctx: &Context, all: bool, names: Vec<String>) -> CoreResult<()> {
    font_caps::install(ctx.dry_run(), ctx.is_interactive(), all, names).await
}

pub async fn uninstall(ctx: &Context, all: bool, names: Vec<String>) -> CoreResult<()> {
    font_caps::uninstall(ctx.dry_run(), ctx.is_interactive(), all, names).await
}

pub async fn update(ctx: &Context) -> CoreResult<()> {
    font_caps::update(ctx.dry_run()).await
}

pub async fn list(ctx: &Context, installed: bool, all: bool) -> CoreResult<()> {
    let rows = font_caps::font_list_rows(installed, all);

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
                        "installed".to_string()
                    } else {
                        "not installed".to_string()
                    },
                ]
            })
            .collect(),
    );
    let output = ctx.ui().new_output_content().table(None, table);
    ctx.ui().print(&output)
}
