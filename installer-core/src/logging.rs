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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Human,
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Human
    }
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
    let base_level = if verbose {
        LevelFilter::DEBUG
    } else {
        config.level_filter()
    };

    let level_directive = base_level.to_string().to_lowercase();
    let env_filter = if let Ok(filter) = EnvFilter::try_from_default_env() {
        filter
    } else {
        EnvFilter::new(level_directive)
    };

    let writer = make_writer(config.file.as_ref())?;
    let subscriber: Box<dyn Subscriber + Send + Sync> = match config.format {
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
    };

    tracing::subscriber::set_global_default(subscriber)
        .context("initializing global logging subscriber")?;
    Ok(())
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
        self.inner.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.lock().unwrap().flush()
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
