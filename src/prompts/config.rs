//! Contains the configurations for prompts used throughout `skeletons`.

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Get the configuration settings for the authentication prompts.
pub fn get_authentication_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(Color::DarkGreen);
    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("🤔").with_fg(Color::DarkRed));
    render_config.help_message = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::DarkYellow);
    render_config.prompt_prefix = Styled::new("☠️ ");

    render_config
}
