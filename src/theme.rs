use crate::api::{Message, Role};
use console::style;
use dialoguer::theme::Theme;
use textwrap::wrap;

pub struct LLMTheme;

impl Theme for LLMTheme {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        let styled_prompt = style(prompt).bold().underlined();
        write!(f, "{}\n> ", styled_prompt)
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        let styled_prompt = style(prompt).green().bold().underlined();
        match default {
            Some(default) if prompt.is_empty() => write!(f, "[{}]\n> ", default),
            Some(default) => write!(f, "{} [{}]\n> ", styled_prompt, default),
            None => write!(f, "{}\n> ", styled_prompt),
        }
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        let styled_prompt = style(prompt).green().bold().underlined();
        write!(f, "{}\n{}", styled_prompt, wrap(sel, 72).join("\n"))
    }
}

fn format_role(role: &Role) -> String {
    let role = match role {
        Role::System => style("System"),
        Role::User => style("User").green(),
        Role::Assistant => style("Assistant").blue(),
    };
    format!("{}", role.bold().underlined())
}

pub fn format_message(message: &Message, width: usize) -> String {
    let role = format_role(&message.role);
    let message = wrap(&message.content, width).join("\n");
    format!("{}\n{}", role, message)
}
