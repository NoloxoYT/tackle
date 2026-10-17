// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
use std::process::{Command, Stdio};

use anyhow::{Context, bail};
use serde::{Deserialize, Serialize};

/// The abstraction layer between User and Package
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageCommand {
    base: String,
    need_additional_input: bool,
    additional_input_description: String,
    show_output: bool,
}

impl PackageCommand {
    /// Run the command without any additional input.
    pub fn run(self) -> anyhow::Result<()> {
        self.execute(None)
    }

    /// Run the command, replacing the first `${ARG}` with `additional_input`.
    #[allow(dead_code)] // The TUI does not collect `${ARG}` input yet.
    pub fn run_with_input<S: ToString>(self, additional_input: S) -> anyhow::Result<()> {
        self.execute(Some(additional_input.to_string()))
    }

    fn execute(self, additional_input: Option<String>) -> anyhow::Result<()> {
        let command = if self.need_additional_input {
            let Some(input) = additional_input else {
                bail!(
                    "`{}` requires additional input: {}",
                    self.base,
                    self.additional_input_description
                );
            };
            if !self.base.contains("${ARG}") {
                bail!("`{}` is missing the ${{ARG}} placeholder", self.base);
            }
            self.base.replacen("${ARG}", &input, 1)
        } else {
            self.base
        };

        let mut process = Command::new("sh");
        process.arg("-c").arg(&command);

        if !self.show_output {
            process.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let status = process
            .status()
            .with_context(|| format!("Failed to run `{command}`"))?;
        if !status.success() {
            bail!("Command `{command}` exited with {status}");
        }

        Ok(())
    }
}
