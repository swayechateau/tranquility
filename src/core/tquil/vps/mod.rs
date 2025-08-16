// Module: Command/Vps
// Location: cli/src/command/vps/mod.rs
pub mod add;
pub mod connect;
pub mod delete;
pub mod list;
pub mod update;
pub mod script;
pub mod vps;

pub use vps::fix_vps;