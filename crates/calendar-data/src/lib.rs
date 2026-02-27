/// Calendar data crate - will hold TOML data files and parsing logic.
/// Phase 1: data is embedded directly in calendar-core's sanctoral module.
/// Phase 2+ will move data here as TOML files.

pub fn version() -> &'static str {
    "0.1.0"
}
