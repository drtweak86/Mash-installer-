use anyhow::Result;

use crate::InstallContext;

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

    crate::pkg::ensure_packages(ctx.driver, &pkgs, ctx.dry_run)?;
    Ok(())
}
