use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::font as font_caps;
use std::io::IsTerminal;

fn can_prompt(ctx: &Context) -> bool {
    ctx.is_interactive() && std::io::stdin().is_terminal() && std::io::stderr().is_terminal()
}

pub async fn install(ctx: &Context, all: bool, names: Vec<String>) -> CoreResult<()> {
    font_caps::install(ctx.dry_run(), can_prompt(ctx), all, names).await
}

pub async fn uninstall(ctx: &Context, all: bool, names: Vec<String>) -> CoreResult<()> {
    font_caps::uninstall(ctx.dry_run(), can_prompt(ctx), all, names).await
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
        // Structured formats still emit an (empty) table so output stays parseable.
        if !ctx.ui().is_structured() {
            return Ok(());
        }
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
