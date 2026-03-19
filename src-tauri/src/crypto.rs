use keyring::Entry;
use crate::error::AppError;

const SERVICE_NAME: &str = "murmure2";
const USERNAME: &str = "api_key";

pub fn store_api_key(key: &str) -> Result<(), AppError> {
    let entry = Entry::new(SERVICE_NAME, USERNAME)
        .map_err(|e| AppError::ConfigError(format!("Failed to create keyring entry: {}", e)))?;
    
    entry.set_password(key)
        .map_err(|e| AppError::ConfigError(format!("Failed to store API key: {}", e)))?;
    
    Ok(())
}

pub fn retrieve_api_key() -> Result<String, AppError> {
    let entry = Entry::new(SERVICE_NAME, USERNAME)
        .map_err(|e| AppError::ConfigError(format!("Failed to create keyring entry: {}", e)))?;
    
    let password = entry.get_password()
        .map_err(|e| AppError::ConfigError(format!("Failed to retrieve API key: {}", e)))?;
    
    Ok(password)
}

pub fn delete_api_key() -> Result<(), AppError> {
    let entry = Entry::new(SERVICE_NAME, USERNAME)
        .map_err(|e| AppError::ConfigError(format!("Failed to create keyring entry: {}", e)))?;
    
    entry.delete_credential()
        .map_err(|e| AppError::ConfigError(format!("Failed to delete API key: {}", e)))?;
    
    Ok(())
}

pub fn has_api_key() -> bool {
    let entry = Entry::new(SERVICE_NAME, USERNAME);
    match entry {
        Ok(entry) => {
            match entry.get_password() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}