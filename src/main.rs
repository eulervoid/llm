mod api;
use std::time::Duration;

use api::{Message, OpenAI, Role};
use clap::Parser;
use dialoguer::{theme::SimpleTheme, Input};
use indicatif::ProgressBar;
use reqwest::Error;

#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(short, long)]
    /// OpenAI Model identifier (default: "gpt-3.5-turbo")
    model: Option<String>,
    #[clap(short, long)]
    /// System message (default: "You are a helpful assistant.")
    system_message: Option<String>,
    /// Initial user message
    #[clap(short, long)]
    prompt: Option<String>,
    /// Start an interactive session
    #[clap(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    let api_key = std::env::var("OPENAI_API_KEY").expect("Error: OPENAI_API_KEY is not defined");
    let api = OpenAI::new(&api_key);

    let mut messages = vec![Message {
        role: Role::System,
        content: opts
            .system_message
            .unwrap_or_else(|| "You are a helpful assistant.".into()),
    }];

    if let Some(user_message) = opts.prompt {
        messages.push(Message {
            role: Role::User,
            content: user_message,
        });
    }

    loop {
        if !messages.is_empty() && messages.last().unwrap().role != Role::User {
            let user_message: String = Input::<String>::with_theme(&SimpleTheme)
                .with_prompt("User")
                .interact_text()
                .unwrap();
            messages.push(Message {
                role: Role::User,
                content: user_message,
            });
        }
        let spinner = ProgressBar::new_spinner();
        spinner.set_message("processing");
        spinner.enable_steady_tick(Duration::from_millis(50));
        let result = api
            .get_chat_completion(opts.model.as_deref().unwrap_or("gpt-3.5-turbo"), &messages)
            .await
            .unwrap();
        spinner.finish_and_clear();
        let assistant_message = &result.choices.first().unwrap().message;
        println!("{}", format_assistant_message(&assistant_message.content));
        messages.push(assistant_message.to_owned());
        if !opts.interactive {
            break;
        }
    }

    Ok(())
}

fn format_assistant_message(message: &str) -> String {
    format!("Assistant: {}", message)
}
