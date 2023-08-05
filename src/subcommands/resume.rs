//! Contains all functionality pertaining to the `auto-resume` subcommand.

use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::{Component, Path},
};

use ansi_term::Color;
use chatgpt::prelude::ChatGPT;
use lazy_static::lazy_static;
use ptree::TreeBuilder;
use termimad::{rgb, FmtText, MadSkin};

use crate::{errors::FettersError, models::config::FettersSettings};

lazy_static! {
    /// The current user's username.
    static ref USERNAME: String = whoami::username();

    /// The path to the `.bash_history` file.
    static ref BASH_HISTORY_PATH: String = {
        if cfg!(any(target_os = "linux", target_os = "windows")) {
            format!("/home/{}/.bash_history", *USERNAME)
        } else {
            format!("/users/{}/.bash_history", *USERNAME)
        }
    };
    /// The path to the `fish_history` file.
    static ref FISH_HISTORY_PATH: String = {
        if cfg!(any(target_os = "linux", target_os = "windows")) {
            format!("/home/{}/.local/share/fish/fish_history", *USERNAME)
        } else {
            format!("/users/{}/.local/share/fish/fish_history", *USERNAME)
        }
    };
    /// The path to the `.zsh_history` file.
    static ref ZSH_HISTORY_PATH: String = {
        if cfg!(any(target_os = "linux", target_os = "windows")) {
            format!("/home/{}/.zsh_history", *USERNAME)
        } else {
            format!("/users/{}/.zsh_history", *USERNAME)
        }
    };

    /// The `MadSkin` that is applied to the Markdown resume.
    static ref MADSKIN: MadSkin = {
        let mut skin = MadSkin::default();
        skin.set_headers_fg(rgb(183, 65, 14));

        skin
    };
    /// The filename used when saving the resume to disk.
    static ref RESUME_NAME: &'static str = "fetters_autogen_resume.md";
}

/// Generate a resume based on command-line history. Display the Markdown resume once a response is
/// returned from the ChatGPT API. Saves the Markdown to file if `save_to` is provided.
pub async fn generate_resume(
    fetters_settings: &FettersSettings,
    save_to: Option<String>,
) -> Result<(), FettersError> {
    let api_key = fetters_settings
        .chatgpt
        .api_key
        .clone()
        .ok_or(FettersError::GenericError(
            "Missing ChatGPT API key! Cannot auto-generate a resume for you.".to_string(),
        ))?;

    let consolidated_history = read_history()?;

    if !consolidated_history.is_empty() {
        let markdown_resume = get_chatgpt_resume(consolidated_history, api_key).await?;
        let formatted_markdown = FmtText::from(&MADSKIN, &markdown_resume, Some(100));

        println!("\n{formatted_markdown}");

        if let Some(path) = save_to {
            let directory_path = Path::new(&path);

            if !directory_path.exists() || !directory_path.is_dir() {
                println!(
                    "{}",
                    Color::Fixed(172)
                        .bold()
                        .paint("⚠️  Please enter a valid directory path!")
                );
            } else {
                let save_to = directory_path.join(RESUME_NAME.to_string());

                let resume_file = File::create(&save_to)?;
                let mut buffered_writer = BufWriter::new(resume_file);
                buffered_writer.write_all(markdown_resume.as_bytes())?;

                println!(
                    "{}",
                    Color::Green
                        .bold()
                        .paint("✅ Successfully saved the resume!\n")
                );
                display_save_to_path(&save_to)?;
            }
        }
    } else {
        println!(
            "{}",
            Color::Fixed(172).bold().paint(
                "⚠️  Did not find any shell history at the following locations on your machine:"
            )
        );
        println!("Bash history | {}", *BASH_HISTORY_PATH);
        println!("Fish history | {}", *FISH_HISTORY_PATH);
        println!("Zsh history  | {}", *ZSH_HISTORY_PATH);
    }

    Ok(())
}

/// Extracts the contents of a file to a byte buffer, then converts the byte buffer into a UTF-8
/// string.
fn read_file_to_string(path: String) -> Result<String, FettersError> {
    let mut file = File::open(path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(String::from_utf8_lossy(&buffer).to_string())
}

/// Get the contents of the history files and return a string that consolidates all three history
/// files together.
fn read_history() -> Result<String, FettersError> {
    let bash_history = read_file_to_string(BASH_HISTORY_PATH.to_string()).unwrap_or("".to_string());
    let fish_history = read_file_to_string(FISH_HISTORY_PATH.to_string()).unwrap_or("".to_string());
    let zsh_history = read_file_to_string(ZSH_HISTORY_PATH.to_string())?;

    let consolidated_history = format!("{bash_history}{fish_history}{zsh_history}");

    Ok(consolidated_history)
}

/// Make a request to ChatGPT to generate a Markdown resume based on command-line history.
async fn get_chatgpt_resume(
    api_key: String,
    consolidated_history: String,
) -> Result<String, FettersError> {
    let chatgpt_client = ChatGPT::new(api_key)?;

    let message = format!(
        "Generate a resume in Markdown from the given UNIX command-line history that includes Bash, Zsh, and Fish history: {consolidated_history}"
    );

    let markdown_resume = &chatgpt_client
        .send_message(message)
        .await?
        .message()
        .content
        .clone();

    Ok(markdown_resume.to_string())
}

/// Display a tree denoting the location at which the Markdown resume was saved to.
fn display_save_to_path(path: &Path) -> Result<(), FettersError> {
    let mut tree_builder = TreeBuilder::new(format!(
        "{}",
        Color::RGB(183, 65, 14)
            .bold()
            .underline()
            .paint("📄 Resume location")
    ));

    for component in path.components() {
        if let Component::Normal(name) = component {
            let painted_item = if name.to_string_lossy() == RESUME_NAME.to_string() {
                format!(
                    "{}",
                    Color::White
                        .bold()
                        .paint(name.to_string_lossy().to_string())
                )
            } else {
                format!(
                    "{}",
                    Color::Blue.bold().paint(name.to_string_lossy().to_string())
                )
            };

            tree_builder.begin_child(painted_item);
        }
    }

    tree_builder.end_child();

    let tree = tree_builder.build();
    ptree::print_tree(&tree)?;
    println!();

    Ok(())
}
