use tauri::{plugin::{Builder, TauriPlugin}, runtime::Runtime};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("kree").build()
}
