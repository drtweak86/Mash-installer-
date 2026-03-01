//! Pi Overlord Phase — Cross-distro package installation

use anyhow::Result;

use crate::context::PhaseContext;
use crate::driver::DistroDriver;
use crate::pi_overlord::{PackageCategory, PiOverlord};

/// Pi Overlord installation phase
pub async fn install_phase(ctx: &mut PhaseContext<'_>, driver: &dyn DistroDriver) -> Result<()> {
    ctx.record_action("🧠 Starting Pi Overlord cross-distro package installation...");

    // Create Pi Overlord instance
    let pi_overlord = PiOverlord::new();

    // Run the full installation sequence
    pi_overlord.run_full_sequence(driver, ctx).await?;

    ctx.record_action("✅ Pi Overlord installation completed successfully!");
    ctx.record_action("🎉 Your system now has a comprehensive development environment!");

    Ok(())
}

/// Install specific Pi Overlord category
pub async fn install_category_phase(
    ctx: &mut PhaseContext<'_>,
    driver: &dyn DistroDriver,
    category: PackageCategory,
) -> Result<()> {
    ctx.record_action(format!(
        "📦 Installing Pi Overlord category: {:?}",
        category
    ));

    let pi_overlord = PiOverlord::new();
    pi_overlord.install_category(category, driver, ctx).await?;

    ctx.record_action(format!("✅ Completed installation of {:?}", category));

    Ok(())
}
