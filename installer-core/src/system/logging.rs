#![allow(dead_code)]

use anyhow::{Context, Result};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing_subscriber::{
    filter::LevelFilter, fmt, fmt::format::FmtSpan, fmt::writer::BoxMakeWriter, prelude::*,
    EnvFilter,
};

fn level_filter(config: &crate::model::config::LoggingConfig) -> LevelFilter {
    match config.level.to_lowercase().as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG,
        "info" => LevelFilter::INFO,
        "warn" | "warning" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    }
}

fn build_env_filter(config: &crate::model::config::LoggingConfig, verbose: bool) -> EnvFilter {
    let base_level = if verbose {
        LevelFilter::DEBUG
    } else {
        level_filter(config)
    };

    let level_directive = base_level.to_string().to_lowercase();
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        filter
    } else {
        EnvFilter::new(level_directive)
    }
}

pub fn init(config: &crate::model::config::LoggingConfig, verbose: bool) -> Result<()> {
    let log_file = config
        .file
        .clone()
        .or_else(|| dirs::home_dir().map(|h| h.join("mash-install.log")));

    let env_filter = build_env_filter(config, verbose);
    let mut layers = Vec::new();

    // 1. File Layer (Always enabled if path is resolvable)
    if let Some(path) = log_file {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(&path) {
            let guard = Arc::new(Mutex::new(file));
            let writer = BoxMakeWriter::new(move || LockedWriter {
                inner: guard.clone(),
            });
            let file_layer = fmt::layer()
                .with_writer(writer)
                .event_format(fmt::format().compact())
                .with_ansi(false)
                .with_span_events(FmtSpan::CLOSE)
                .boxed();
            layers.push(file_layer);
        }
    }

    // 2. Stdout Layer (Only if verbose or no file layer)
    if verbose || layers.is_empty() {
        let stdout_layer = fmt::layer()
            .with_writer(io::stdout)
            .event_format(fmt::format().compact())
            .boxed();
        layers.push(stdout_layer);
    }

    let subscriber = tracing_subscriber::registry().with(env_filter).with(layers);

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
        let config = crate::model::config::LoggingConfig {
            level: "info".into(),
            format: crate::model::config::LogFormat::Json,
            file: Some(log_path.clone()),
        };

        let env_filter = build_env_filter(&config, false);
        let file = File::create(&log_path)?;
        let writer = Arc::new(Mutex::new(file));
        let layer = fmt::layer()
            .with_writer(move || LockedWriter {
                inner: writer.clone(),
            })
            .json()
            .boxed();
        let subscriber = tracing_subscriber::registry().with(env_filter).with(layer);
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
        let config = crate::model::config::LoggingConfig {
            level: "warn".into(),
            format: crate::model::config::LogFormat::Human,
            file: Some(log_path.clone()),
        };

        let env_filter = build_env_filter(&config, false);
        let file = File::create(&log_path)?;
        let writer = Arc::new(Mutex::new(file));
        let layer = fmt::layer()
            .with_writer(move || LockedWriter {
                inner: writer.clone(),
            })
            .boxed();
        let subscriber = tracing_subscriber::registry().with(env_filter).with(layer);
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
