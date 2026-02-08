use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    installer_core::run().context("installer failed")
}
