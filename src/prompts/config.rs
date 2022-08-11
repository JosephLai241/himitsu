//! Contains the configurations for prompts used throughout `himitsu`.

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Variants for configuration types to use in the application.
pub enum ConfigType {
    /// Get the confirmation prompt configuration for the application.
    Confirm,
    /// Get the standard configuration for the application.
    Standard,
}

/// Get the configuration settings for the application.
pub fn get_inquire_config(config_type: ConfigType, highlight_answer: bool) -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.answer = if highlight_answer {
        StyleSheet::new()
            .with_attr(Attributes::BOLD)
            .with_fg(Color::DarkGreen)
    } else {
        StyleSheet::new()
    };
    render_config.error_message = render_config
        .error_message
        .with_message(
            StyleSheet::new()
                .with_attr(Attributes::BOLD)
                .with_fg(Color::DarkRed),
        )
        .with_prefix(Styled::new("🤔"));
    render_config.help_message = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(Color::DarkYellow);
    render_config.prompt_prefix = Styled::new("㊙");

    match config_type {
        ConfigType::Confirm => {
            render_config.prompt = render_config
                .prompt
                .with_attr(Attributes::BOLD)
                .with_fg(Color::DarkYellow)
        }
        _ => {}
    }

    render_config
}
