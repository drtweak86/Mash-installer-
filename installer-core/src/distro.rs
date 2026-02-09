use crate::driver::DistroDriver;

pub fn translate_names(driver: &dyn DistroDriver, pkgs: &[&str]) -> Vec<String> {
    pkgs.iter()
        .filter_map(|pkg| driver.translate_package(pkg))
        .collect()
}
