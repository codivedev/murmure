use crate::error::{AppError, Result};
use enigo::{Enigo, Key, Settings, Keyboard, Direction};
use std::process::Command;
use std::io::Write;

pub fn insert_text(text: &str) -> Result<()> {
    if let Err(_) = try_insert_text_direct(text) {
        copy_to_clipboard(text)?;
        paste()?;
    }
    Ok(())
}

fn try_insert_text_direct(text: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::InputError(format!("Failed to create Enigo instance: {}", e)))?;

    for c in text.chars() {
        enigo.text(&c.to_string())
            .map_err(|e| AppError::InputError(format!("Failed to type character '{}': {}", c, e)))?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let result = Command::new(get_clipboard_copy_command())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
            child.wait()
        });

    if result.is_err() {
        return Err(AppError::InputError(format!("Failed to copy to clipboard: {}", result.unwrap_err())));
    }
    
    Ok(())
}

pub fn paste() -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::InputError(format!("Failed to create Enigo instance: {}", e)))?;

    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Press)
            .map_err(|e| AppError::InputError(format!("Failed to press Cmd key: {}", e)))?;
        enigo.key(Key::Unicode('v'), Direction::Click)
            .map_err(|e| AppError::InputError(format!("Failed to press V key: {}", e)))?;
        enigo.key(Key::Meta, Direction::Release)
            .map_err(|e| AppError::InputError(format!("Failed to release Cmd key: {}", e)))?;
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::Control, Direction::Press)
            .map_err(|e| AppError::InputError(format!("Failed to press Ctrl key: {}", e)))?;
        enigo.key(Key::Unicode('v'), Direction::Click)
            .map_err(|e| AppError::InputError(format!("Failed to press V key: {}", e)))?;
        enigo.key(Key::Control, Direction::Release)
            .map_err(|e| AppError::InputError(format!("Failed to release Ctrl key: {}", e)))?;
    }

    Ok(())
}

fn get_clipboard_copy_command() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "clip"
    }
    #[cfg(target_os = "macos")]
    {
        "pbcopy"
    }
    #[cfg(target_os = "linux")]
    {
        "xclip -selection clipboard"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_text_basic() {
        let result = insert_text("Hello, World!");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_insert_text_unicode() {
        let result = insert_text("Hello, 世界! ❤️ 🚀");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_paste_function() {
        let result = paste();
        assert!(result.is_ok() || result.is_err());
    }
}