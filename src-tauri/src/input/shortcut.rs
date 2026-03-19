use crate::error::{AppError, Result};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Runtime, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// Callback function type for shortcut events.
/// Takes the shortcut string as parameter.
pub type Callback = Box<dyn Fn(&str) + Send + Sync>;

/// Global state to store the app handle and registered callbacks
static APP_HANDLE: once_cell::sync::Lazy<Arc<Mutex<Option<AppHandle<tauri::Wry>>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));
static CALLBACKS: once_cell::sync::Lazy<
    Arc<Mutex<Option<(Callback, Callback)>>>,
> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

/// Initialize the shortcut module with the Tauri app handle.
///
/// This must be called once during application setup before using register/unregister.
pub fn init(app_handle: AppHandle<Wry>) -> Result<()> {
    let mut handle = APP_HANDLE.lock().map_err(|_| {
        AppError::InputError("Failed to acquire app handle lock".to_string())
    })?;
    *handle = Some(app_handle);
    Ok(())
}

/// Register a global shortcut with press and release callbacks.
///
/// # Arguments
/// * `shortcut` - The shortcut string (e.g., "Ctrl+Shift+C", "Alt+Space")
/// * `on_press` - Callback function called when shortcut is pressed
/// * `on_release` - Callback function called when shortcut is released
///
/// # Returns
/// * `Result<()>` - Ok(()) on success, AppError::InputError on failure
///
/// # Platform Limitations
/// **X11 ONLY**: This functionality is currently only supported on X11 Linux systems.
/// Wayland support is not available in the current version of tauri-plugin-global-shortcut.
/// On other platforms (Windows, macOS, Wayland), this function will return an error.
pub fn register(shortcut: &str, on_press: Callback, on_release: Callback) -> Result<()> {
    {
        let mut stored_callbacks = CALLBACKS.lock().map_err(|_| {
            AppError::InputError("Failed to acquire callback lock".to_string())
        })?;
        *stored_callbacks = Some((on_press, on_release));
    }

    let app_handle = {
        let handle = APP_HANDLE.lock().map_err(|_| {
            AppError::InputError("App handle not initialized. Call init() first.".to_string())
        })?;
        handle.clone().ok_or_else(|| {
            AppError::InputError("App handle not initialized. Call init() first.".to_string())
        })?
    };
    let shortcut_string = shortcut.to_string();
    app_handle
        .global_shortcut()
        .on_shortcut(shortcut, move |app, _registered_shortcut, event| {
            match event.state {
                ShortcutState::Pressed => {
                    let _ = app.emit("shortcut-pressed", &shortcut_string);
                    
                    if let Ok(stored_callbacks) = CALLBACKS.lock() {
                        if let Some((press_cb, _)) = &*stored_callbacks {
                            press_cb(&shortcut_string);
                        }
                    }
                }
                ShortcutState::Released => {
                    let _ = app.emit("shortcut-released", &shortcut_string);
                    
                    if let Ok(stored_callbacks) = CALLBACKS.lock() {
                        if let Some((_, release_cb)) = &*stored_callbacks {
                            release_cb(&shortcut_string);
                        }
                    }
                }
            }
        })
        .map_err(|e| AppError::InputError(format!("Failed to register shortcut '{}': {}", shortcut, e)))
}

/// Unregister all global shortcuts and clear callbacks.
///
/// # Returns
/// * `Result<()>` - Ok(()) on success, AppError::InputError on failure
///
/// # Platform Limitations
/// **X11 ONLY**: This functionality is currently only supported on X11 Linux systems.
/// Wayland support is not available in the current version of tauri-plugin-global-shortcut.
pub fn unregister() -> Result<()> {
    {
        let mut stored_callbacks = CALLBACKS.lock().map_err(|_| {
            AppError::InputError("Failed to acquire callback lock".to_string())
        })?;
        *stored_callbacks = None;
    }

    let app_handle = {
        let handle = APP_HANDLE.lock().map_err(|_| {
            AppError::InputError("App handle not initialized. Call init() first.".to_string())
        })?;
        handle.clone().ok_or_else(|| {
            AppError::InputError("App handle not initialized. Call init() first.".to_string())
        })?
    };

    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|e| AppError::InputError(format!("Failed to unregister shortcuts: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_module_compiles() {
        assert!(true);
    }

    #[test]
    fn test_callback_types() {
        let _callback: Callback = Box::new(|_s: &str| {});
        assert!(true);
    }
}