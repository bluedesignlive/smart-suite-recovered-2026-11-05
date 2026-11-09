//! The apps are frontends to the OS: they live in the workspace-level `apps/` directory and plug into the shell
//! through `AppId` + the shared `smart-ui` kit. Nothing in `apps/` knows how windows are composited.
#[path = "../../../../apps/files/mod.rs"]
pub mod files;
#[path = "../../../../apps/photos/mod.rs"]
pub mod photos;
#[path = "../../../../apps/terminal/mod.rs"]
pub mod terminal;
#[path = "../../../../apps/settings/mod.rs"]
pub mod settings;
#[path = "../../../../apps/capture/mod.rs"]
pub mod capture;
#[path = "../../../../apps/assistant/mod.rs"]
pub mod assistant;
