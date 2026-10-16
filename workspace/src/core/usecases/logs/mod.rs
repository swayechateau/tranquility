use crate::core::Context;
use crate::core::CoreResult;
use crate::engine::capabilities::logs as logs_caps;

pub async fn show(
    _ctx: &Context,
    tail: usize,
    level: String,
    json_only: bool,
    date: Option<String>,
    path_only: bool,
) -> CoreResult<()> {
    logs_caps::show(tail, level, json_only, date, path_only).await
}
