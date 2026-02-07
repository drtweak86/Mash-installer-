use anyhow::Result;

use crate::InstallContext;

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    // Canonical (Debian) names – pkg.rs translates for Arch automatically
    let pkgs = [
        "fonts-terminus",
        "fonts-noto-color-emoji",
        "xfonts-terminus",
    ];

    crate::pkg::ensure_packages(&pkgs, ctx.dry_run)?;

    tracing::info!(
        "Nerd Fonts are not available via system packages. \
         See https://www.nerdfonts.com/ to install patched terminal fonts."
    );

    Ok(())
}
