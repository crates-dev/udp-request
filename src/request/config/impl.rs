use crate::*;

/// Default implementation for Config.
impl Default for Config {
    /// Creates a default Config instance.
    ///
    /// # Returns
    ///
    /// - `Config` - Default configuration with empty host, default port, timeout and buffer size.
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            host: EMPTY_STR.to_owned(),
            port: DEFAULT_WEB_PORT,
        }
    }
}
