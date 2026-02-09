use anyhow::Result;

use crate::{package_manager, InstallContext};

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    // Canonical (Debian) names – pkg.rs translates for Arch automatically
    let pkgs = [
        "fonts-terminus",
        "fonts-noto-color-emoji",
        "xfonts-terminus",
    ];

    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)?;

    tracing::info!(
        "Nerd Fonts are not available via system packages. \
         See https://www.nerdfonts.com/ to install patched terminal fonts."
    );

    Ok(())
}
