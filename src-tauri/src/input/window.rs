//! Active Window Detection for X11
//!
//! This module provides functionality to detect the currently active window
//! on X11-based systems using the Extended Window Manager Hints (EWMH)
//! specification.
//!
//! **LIMITATION**: This implementation is X11-only and will not work on
//! Wayland or other display servers. On non-X11 systems, the functions
//! will return appropriate errors.

use crate::error::{AppError, Result};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, Window};

/// Information about an active window
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowInfo {
    /// The window title
    pub title: String,
    /// The window class (typically "instance\0class" format)
    pub class: String,
}

/// Get the currently active window information
///
/// This function connects to the X11 server and retrieves the active window
/// using the `_NET_ACTIVE_WINDOW` property from the root window. It then
/// fetches the window title and class properties.
///
/// # Returns
/// - `Ok(WindowInfo)` with the window title and class if successful
/// - `Err(AppError::InputError)` if there are connection issues, protocol errors,
///   or if the required properties cannot be retrieved
///
/// # Errors
/// This function can fail due to:
/// - X11 connection failures
/// - Missing `_NET_ACTIVE_WINDOW` support
/// - Invalid window IDs
/// - Missing or malformed window properties
pub fn get_active_window() -> Result<WindowInfo> {
    // Connect to the X11 server
    let (conn, screen_num) = x11rb::connect(None)
        .map_err(|e| AppError::InputError(format!("Failed to connect to X11 server: {}", e)))?;

    let screen = conn
        .setup()
        .roots
        .get(screen_num)
        .ok_or_else(|| AppError::InputError("Invalid screen number".to_string()))?;

    let root_window = screen.root;

    // Get the _NET_ACTIVE_WINDOW atom
    let net_active_window_atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .map_err(|e| AppError::InputError(format!("Failed to intern _NET_ACTIVE_WINDOW atom: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_ACTIVE_WINDOW atom reply: {}", e)))?
        .atom;

    // Get the active window ID from the root window's _NET_ACTIVE_WINDOW property
    let active_window_reply = conn
        .get_property(
            false,
            root_window,
            net_active_window_atom,
            AtomEnum::WINDOW,
            0,
            1,
        )
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_ACTIVE_WINDOW property: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_ACTIVE_WINDOW reply: {}", e)))?;

    let active_window_id = active_window_reply
        .value32()
        .and_then(|mut iter| iter.next())
        .ok_or_else(|| AppError::InputError("No active window found".to_string()))?;

    if active_window_id == 0 {
        return Err(AppError::InputError("Active window ID is zero".to_string()));
    }

    // Get window title - try _NET_WM_NAME first, then WM_NAME as fallback
    let title = get_window_title(&conn, active_window_id)?;

    // Get window class
    let class = get_window_class(&conn, active_window_id)?;

    Ok(WindowInfo { title, class })
}

/// Get the window title using EWMH _NET_WM_NAME or fallback to WM_NAME
fn get_window_title(conn: &impl Connection, window: Window) -> Result<String> {
    // Try _NET_WM_NAME first (UTF-8)
    let net_wm_name_atom = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .map_err(|e| AppError::InputError(format!("Failed to intern _NET_WM_NAME atom: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_WM_NAME atom reply: {}", e)))?
        .atom;

    let utf8_string_atom = conn
        .intern_atom(false, b"UTF8_STRING")
        .map_err(|e| AppError::InputError(format!("Failed to intern UTF8_STRING atom: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get UTF8_STRING atom reply: {}", e)))?
        .atom;

    let net_wm_name_reply = conn
        .get_property(false, window, net_wm_name_atom, utf8_string_atom, 0, 1024)
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_WM_NAME property: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get _NET_WM_NAME reply: {}", e)))?;

    if !net_wm_name_reply.value.is_empty() {
        return String::from_utf8(net_wm_name_reply.value)
            .map_err(|e| AppError::InputError(format!("Invalid UTF-8 in _NET_WM_NAME: {}", e)));
    }

    // Fallback to WM_NAME (STRING)
    let wm_name_reply = conn
        .get_property(
            false,
            window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            0,
            1024,
        )
        .map_err(|e| AppError::InputError(format!("Failed to get WM_NAME property: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get WM_NAME reply: {}", e)))?;

    if !wm_name_reply.value.is_empty() {
        return String::from_utf8(wm_name_reply.value)
            .map_err(|e| AppError::InputError(format!("Invalid UTF-8 in WM_NAME: {}", e)));
    }

    Ok("".to_string())
}

/// Get the window class (WM_CLASS property)
fn get_window_class(conn: &impl Connection, window: Window) -> Result<String> {
    let wm_class_reply = conn
        .get_property(
            false,
            window,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            0,
            1024,
        )
        .map_err(|e| AppError::InputError(format!("Failed to get WM_CLASS property: {}", e)))?
        .reply()
        .map_err(|e| AppError::InputError(format!("Failed to get WM_CLASS reply: {}", e)))?;

    if wm_class_reply.value.is_empty() {
        return Ok("".to_string());
    }

    // WM_CLASS contains two null-terminated strings: instance and class
    // We'll return the entire raw value as a string for now
    String::from_utf8(wm_class_reply.value)
        .map_err(|e| AppError::InputError(format!("Invalid UTF-8 in WM_CLASS: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_info_struct() {
        let info = WindowInfo {
            title: "Test Title".to_string(),
            class: "Test.Class".to_string(),
        };
        assert_eq!(info.title, "Test Title");
        assert_eq!(info.class, "Test.Class");
    }

    #[test]
    #[ignore = "Requires X11 display server"]
    fn test_get_active_window_integration() {
        // This test requires an actual X11 display server
        // It will be skipped in CI environments without X11
        match get_active_window() {
            Ok(info) => {
                // Should have some data
                assert!(!info.title.is_empty() || !info.class.is_empty());
            }
            Err(e) => {
                // Expected to fail if no X11 server is available
                println!("Expected failure in test environment: {}", e);
            }
        }
    }

    #[test]
    fn test_get_active_window_error_handling() {
        // We can't easily test the actual X11 connection error paths
        // without mocking, but we can at least ensure the function compiles
        // and the error types are correct
        let result = get_active_window();
        match result {
            Ok(_) => {
                // Success case - should have valid data
            }
            Err(AppError::InputError(_)) => {
                // Expected error type
            }
            Err(_) => {
                panic!("Unexpected error type");
            }
        }
    }
}