use anyhow::{Context, Result};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::process::{Command as StdCommand, Output, Stdio};

/// Runs a command and provides detailed errors when it fails.
pub fn run(cmd: &mut StdCommand) -> Result<Output> {
    let desc = describe_command(cmd);
    let output = cmd
        .output()
        .with_context(|| format!("running command: {desc}"))?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(CommandExecutionError::new(desc, output).into())
    }
}

fn describe_command(cmd: &StdCommand) -> String {
    let mut parts = Vec::new();
    parts.push(cmd.get_program().to_string_lossy().into_owned());
    for arg in cmd.get_args() {
        parts.push(arg.to_string_lossy().into_owned());
    }
    parts.join(" ")
}

/// Fluent shell command builder.
pub struct Command {
    inner: StdCommand,
}

impl Command {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            inner: StdCommand::new(program),
        }
    }

    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.inner.arg(arg);
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Self {
        self.inner.args(args);
        self
    }

    pub fn current_dir(mut self, dir: impl AsRef<std::path::Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn env(mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, value);
        self
    }

    pub fn stdin(mut self, cfg: Stdio) -> Self {
        self.inner.stdin(cfg);
        self
    }

    pub fn stdout(mut self, cfg: Stdio) -> Self {
        self.inner.stdout(cfg);
        self
    }

    pub fn stderr(mut self, cfg: Stdio) -> Self {
        self.inner.stderr(cfg);
        self
    }

    pub fn execute(mut self) -> Result<Output> {
        run(&mut self.inner)
    }
}

#[derive(Debug)]
pub struct CommandExecutionError {
    pub command: String,
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandExecutionError {
    fn new(command: String, output: Output) -> Self {
        Self {
            command,
            status: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command '{}' failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            self.command, self.status, self.stdout, self.stderr
        )
    }
}

impl Error for CommandExecutionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_command_includes_arguments() {
        let mut cmd = StdCommand::new("echo");
        cmd.args(["hello", "world"]);
        assert_eq!(describe_command(&cmd), "echo hello world");
    }

    #[test]
    fn run_returns_output_on_success() -> Result<()> {
        let mut cmd = StdCommand::new("echo");
        cmd.arg("ok");
        let output = run(&mut cmd)?;
        assert!(output.status.success());
        Ok(())
    }

    #[test]
    fn run_includes_stdout_and_stderr_on_failure() {
        let mut cmd = StdCommand::new("sh");
        cmd.arg("-c").arg("echo out; echo err >&2; exit 1");
        let err = run(&mut cmd).unwrap_err();
        let err = err
            .downcast_ref::<CommandExecutionError>()
            .expect("expected command error");
        assert!(err.stdout.contains("out"));
        assert!(err.stderr.contains("err"));
    }
}
