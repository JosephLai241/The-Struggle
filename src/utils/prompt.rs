//! Contains a utility function for getting the `RenderConfig` for `inquire` prompts.

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Returns the `RenderConfig` object to use with `inquire` prompts.
pub fn get_inquire_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();

    render_config.error_message = render_config
        .error_message
        .with_message(
            StyleSheet::new()
                .with_attr(Attributes::BOLD)
                .with_fg(Color::DarkRed),
        )
        .with_prefix(Styled::new("🫨"));
    render_config.help_message = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(Color::DarkYellow);
    render_config.prompt_prefix = Styled::new("⛓️");

    render_config
}
