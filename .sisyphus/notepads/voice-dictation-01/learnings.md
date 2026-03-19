# Learnings

## Module Structure Implementation
- Created proper Rust module hierarchy with lib.rs exporting all modules
- Defined shared types with appropriate derives (Debug, Clone, Serialize, Deserialize)
- Added PartialEq for enum types that need comparison capabilities
- Configured Cargo.toml with [lib] section to expose the library
- Used dummy references to avoid unused import warnings during infrastructure setup

## Type Design Patterns
- Used serde derives consistently for all shared types to enable serialization
- Applied documentation comments to clarify which plan each type is associated with
- Structured enums with variants containing appropriate data (e.g., Recording { duration_ms: u64 })
- Used Option types for potentially absent data (language in TranscriptionResult)

## Cargo Configuration
- Added [lib] section to Cargo.toml to expose the library target
- Named the library target distinctly (murmure2_lib) to avoid conflicts
- Maintained all existing dependencies while adding modular structure

## Key Learnings from Settings Storage Module Implementation
- When using tauri-plugin-store or similar persistence mechanisms, proper error handling is crucial
- The `dirs` crate is useful for cross-platform config directory management
- When working with serde_json, proper error conversion is needed to match the application's error type
- Configuration files should have a fallback mechanism to default values when they don't exist
- Default values for settings should be implemented via the Default trait

## Technical Patterns
- SettingsStore pattern with load/save/reset methods provides a clean abstraction for config management
- Using a dedicated config directory (e.g., ~/.config/murmure2/) is standard practice
- JSON serialization for configuration files provides human-readable and editable settings
- Proper error propagation with `?` operator simplifies error handling in config operations