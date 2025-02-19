use super::command::Command;
use crate::cli::Cli;
use crate::config::FnmConfig;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use clap::{IntoApp, Parser};
use clap_complete::{Generator, Shell};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    shell: Option<Shell>,
}

impl Command for Completions {
    type Error = Error;

    fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(Error::CantInferShell)?;
        let app = Cli::command();
        shell.generate(&app, &mut stdio);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "{}\n{}\n{}\n{}",
        "Can't infer shell!",
        "fnm can't infer your shell based on the process tree.",
        "Maybe it is unsupported? we support the following shells:",
        shells_as_string()
    )]
    CantInferShell,
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {x}"))
        .collect::<Vec<_>>()
        .join("\n")
}
