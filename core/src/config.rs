pub struct Config {
    /// Informs if the emulator must run without a
    /// frontend
    pub is_headless: bool,
    /// Informs if it's running in debug mode
    pub is_debug: bool,
    /// Rom name
    pub rom_name: String,
}

impl Config {
    /// Creates a new Config instance from the clap arguments
    pub fn from_clap(matches: clap::ArgMatches) -> Self {
        Self {
            is_headless: matches.occurrences_of("headless") > 0,
            is_debug: matches.occurrences_of("debug") > 0,
            rom_name: matches.value_of("ROM").unwrap().to_string(),
        }
    }
}