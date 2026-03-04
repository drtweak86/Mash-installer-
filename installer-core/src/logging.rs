use crate::{InstallContext, Phase};
use tracing::Span;

/// Create a tracing span for the entire installation process.
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

/// Create a tracing span for an individual installation phase.
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
