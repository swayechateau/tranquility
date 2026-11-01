use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::logs as logs_caps;

pub async fn show(
    ctx: &Context,
    tail: usize,
    level: String,
    json_only: bool,
    date: Option<String>,
    path_only: bool,
) -> CoreResult<()> {
    let log_dir = ctx.base_config().log_directory.clone();
    logs_caps::show(&log_dir, tail, level, json_only, date, path_only).await
}
