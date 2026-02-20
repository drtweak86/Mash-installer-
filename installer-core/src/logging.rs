#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::{Span, Subscriber};
use tracing_subscriber::{
    filter::LevelFilter, fmt, fmt::format::FmtSpan, fmt::writer::BoxMakeWriter, prelude::*,
    EnvFilter,
};

use crate::{InstallContext, Phase};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Human,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub format: LogFormat,
    #[serde(default)]
    pub file: Option<PathBuf>,
}

fn default_log_level() -> String {
    "info".into()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: LogFormat::default(),
            file: None,
        }
    }
}

impl LoggingConfig {
    fn level_filter(&self) -> LevelFilter {
        match self.level.to_lowercase().as_str() {
            "trace" => LevelFilter::TRACE,
            "debug" => LevelFilter::DEBUG,
            "info" => LevelFilter::INFO,
            "warn" | "warning" => LevelFilter::WARN,
            "error" => LevelFilter::ERROR,
            _ => LevelFilter::INFO,
        }
    }
}

pub fn init(config: &LoggingConfig, verbose: bool) -> Result<()> {
    let writer = make_writer(config.file.as_ref())?;
    let subscriber = build_subscriber(config, verbose, writer);

    tracing::subscriber::set_global_default(subscriber)
        .context("initializing global logging subscriber")?;
    Ok(())
}

fn build_env_filter(config: &LoggingConfig, verbose: bool) -> EnvFilter {
    let base_level = if verbose {
        LevelFilter::DEBUG
    } else {
        config.level_filter()
    };

    let level_directive = base_level.to_string().to_lowercase();
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        filter
    } else {
        EnvFilter::new(level_directive)
    }
}

fn build_subscriber(
    config: &LoggingConfig,
    verbose: bool,
    writer: BoxMakeWriter,
) -> Box<dyn Subscriber + Send + Sync> {
    let env_filter = build_env_filter(config, verbose);
    match config.format {
        LogFormat::Json => Box::new(
            tracing_subscriber::registry()
                .with(env_filter.clone())
                .with(
                    fmt::layer()
                        .event_format(fmt::format().json())
                        .with_writer(writer)
                        .with_span_events(FmtSpan::FULL),
                ),
        ),
        LogFormat::Human => Box::new(
            tracing_subscriber::registry()
                .with(env_filter.clone())
                .with(
                    fmt::layer()
                        .event_format(fmt::format().compact())
                        .with_writer(writer)
                        .with_span_events(FmtSpan::FULL),
                ),
        ),
    }
}

fn make_writer(path: Option<&PathBuf>) -> Result<BoxMakeWriter, io::Error> {
    if let Some(dest) = path {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(dest)?;
        let guard = Arc::new(Mutex::new(file));
        Ok(BoxMakeWriter::new(move || LockedWriter {
            inner: guard.clone(),
        }))
    } else {
        Ok(BoxMakeWriter::new(io::stdout))
    }
}

struct LockedWriter {
    inner: Arc<Mutex<File>>,
}

impl Write for LockedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner
            .lock()
            .map_err(|e| io::Error::other(e.to_string()))?
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner
            .lock()
            .map_err(|e| io::Error::other(e.to_string()))?
            .flush()
    }
}

pub fn install_span(ctx: &InstallContext) -> Span {
    tracing::info_span!(
        "install",
        driver = ctx.platform.driver_name,
        profile = ?ctx.options.profile,
        arch = %ctx.platform.platform.arch,
        distro = %ctx.platform.platform.distro,
        staging = %ctx.options.staging_dir.display()
    )
}

pub fn phase_span(ctx: &InstallContext, phase: &dyn Phase) -> Span {
    tracing::info_span!(
        "phase",
        name = phase.name(),
        description = phase.description(),
        severity = ?phase.error_severity(),
        driver = ctx.platform.driver_name,
        profile = ?ctx.options.profile,
        distro = %ctx.platform.platform.distro,
        arch = %ctx.platform.platform.arch,
        staging = %ctx.options.staging_dir.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;
    use tracing::{dispatcher, info, warn};

    #[test]
    fn json_format_records_structured_entries() -> Result<()> {
        let dir = tempdir()?;
        let log_path = dir.path().join("json-log.log");
        let config = LoggingConfig {
            level: "info".into(),
            format: LogFormat::Json,
            file: Some(log_path.clone()),
        };

        let writer = make_writer(config.file.as_ref())?;
        let subscriber = build_subscriber(&config, false, writer);
        let dispatch = dispatcher::Dispatch::new(subscriber);
        dispatcher::with_default(&dispatch, || info!("structured event"));

        drop(dispatch);

        let contents = fs::read_to_string(log_path)?;
        assert!(contents.contains("\"level\":\"INFO\""));
        assert!(contents.contains("\"message\":\"structured event\""));
        Ok(())
    }

    #[test]
    fn level_filter_filters_lower_levels() -> Result<()> {
        let dir = tempdir()?;
        let log_path = dir.path().join("level.log");
        let config = LoggingConfig {
            level: "warn".into(),
            format: LogFormat::Human,
            file: Some(log_path.clone()),
        };

        let writer = make_writer(config.file.as_ref())?;
        let subscriber = build_subscriber(&config, false, writer);
        let dispatch = dispatcher::Dispatch::new(subscriber);
        dispatcher::with_default(&dispatch, || {
            info!("filtered info");
            warn!("visible warn");
        });

        drop(dispatch);

        let contents = fs::read_to_string(log_path)?;
        assert!(contents.contains("warn"));
        assert!(contents.contains("visible warn"));
        assert!(!contents.contains("filtered info"));
        Ok(())
    }
}
