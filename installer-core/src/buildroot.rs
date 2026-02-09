use anyhow::Result;

use crate::{package_manager, InstallContext};

/// Buildroot build dependencies.
pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    let pkgs = [
        "bison",
        "flex",
        "gawk",
        "texinfo",
        "libncurses-dev",
        "libssl-dev",
        "bc",
        "rsync",
        "cpio",
        "unzip",
        "file",
        "patch",
        "python3",
        "python3-pip",
        "python3-venv",
    ];

    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)?;
    Ok(())
}
