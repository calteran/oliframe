//! Configuration module
mod frame_config;
mod input_config;
mod output_config;

use derive_getters::Getters;
pub use frame_config::FrameConfig;
pub use input_config::InputConfig;
pub use output_config::OutputConfig;

/// Program configuration
#[derive(Debug, Getters)]
pub struct Config {
    input_config: InputConfig,
    output_config: OutputConfig,
    task_config: FrameConfig,
}

impl Config {
    /// Creates a new configuration from the given input, output, and task configurations.
    pub fn new(
        input_config: InputConfig,
        output_config: OutputConfig,
        task_config: FrameConfig,
    ) -> Self {
        Self {
            input_config,
            output_config,
            task_config,
        }
    }
}
