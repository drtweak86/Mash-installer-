use anyhow::{Context, Result};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::process::{Command as StdCommand, Output, Stdio};
use tracing::{debug, error};

/// Runs a command and provides detailed errors when it fails.
pub fn run(cmd: &mut StdCommand) -> Result<Output> {
    let program = cmd.get_program().to_string_lossy();
    let is_sudo = program == "sudo" || program.ends_with("/sudo");
    let password = if is_sudo {
        crate::sudo_password::get_sudo_password()
    } else {
        None
    };

    let desc = describe_command(cmd);

    let output = if let Some(pass) = password {
        // Inject -S (stdin) into sudo command if not already present
        let args: Vec<_> = cmd.get_args().collect();
        let has_s = args.iter().any(|a| a.to_string_lossy() == "-S");
        
        let mut new_cmd = StdCommand::new(cmd.get_program());
        if !has_s {
            new_cmd.arg("-S");
        }
        new_cmd.args(args);
        // Copy env and current_dir from original
        for (k, v) in cmd.get_envs() {
            if let Some(v) = v {
                new_cmd.env(k, v);
            } else {
                new_cmd.env_remove(k);
            }
        }
        if let Some(dir) = cmd.get_current_dir() {
            new_cmd.current_dir(dir);
        }

        new_cmd.stdin(Stdio::piped());
        new_cmd.stdout(Stdio::piped());
        new_cmd.stderr(Stdio::piped());

        let mut child = new_cmd.spawn().with_context(|| format!("spawning command: {desc}"))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = writeln!(stdin, "{}", pass);
        }

        child.wait_with_output().with_context(|| format!("waiting for command: {desc}"))?
    } else {
        cmd.output()
            .with_context(|| format!("running command: {desc}"))?
    };

    let details = CommandExecutionDetails::from_output(desc.clone(), &output);

    debug!(
        command = %details.command,
        status = ?details.status,
        stdout = %details.stdout.trim_end(),
        stderr = %details.stderr.trim_end(),
        "command completed"
    );

    if output.status.success() {
        Ok(output)
    } else {
        error!(
            command = %details.command,
            status = ?details.status,
            stdout = %details.stdout.trim_end(),
            stderr = %details.stderr.trim_end(),
            "command failed"
        );
        Err(CommandExecutionError::new(details).into())
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

/// Captured command execution details.
#[derive(Clone, Debug)]
pub struct CommandExecutionDetails {
    pub command: String,
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandExecutionDetails {
    fn from_output(command: String, output: &Output) -> Self {
        Self {
            command,
            status: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }

    pub fn success(&self) -> bool {
        self.status == Some(0)
    }
}

#[derive(Debug)]
pub struct CommandExecutionError {
    pub command: String,
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    details: CommandExecutionDetails,
}

impl CommandExecutionError {
    pub fn new(details: CommandExecutionDetails) -> Self {
        let CommandExecutionDetails {
            command,
            status,
            stdout,
            stderr,
        } = details.clone();
        Self {
            command,
            status,
            stdout,
            stderr,
            details,
        }
    }

    pub fn details(&self) -> &CommandExecutionDetails {
        &self.details
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

/// Standard flags for all `curl` invocations.
///
/// Returns `["-fsSL", "--proto", "=https", "--tlsv1.2"]` to enforce
/// TLS 1.2+ on every download. Use via `.args(curl_flags())`.
pub fn curl_flags() -> &'static [&'static str] {
    &["-fsSL", "--proto", "=https", "--tlsv1.2"]
}

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
    fn curl_flags_includes_tls_enforcement() {
        let flags = curl_flags();
        assert!(flags.contains(&"--proto"), "should include --proto");
        assert!(flags.contains(&"=https"), "should include =https");
        assert!(flags.contains(&"--tlsv1.2"), "should include --tlsv1.2");
        assert!(flags.contains(&"-fsSL"), "should include -fsSL");
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
