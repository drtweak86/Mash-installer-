// Copyright 2024 MASH Installer Authors
// SPDX-License-Identifier: MIT

use anyhow::Result;
use installer_core::{
    command_exists, ensure_i3_installed, ensure_kitty_installed, ensure_retro_theme_dependencies,
    install_retro_theme, install_theme_file, ThemeConfig,
};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_full_theme_workflow() -> Result<()> {
    // Create temporary directory for testing
    let temp_dir = tempdir()?;
    let base_path = temp_dir.path().to_path_buf();

    // Test complete theme installation sequence
    let result = install_retro_theme(&base_path);
    assert!(result.is_ok(), "Theme installation should succeed");

    // Verify all files installed to correct locations
    assert!(
        base_path.join(".config/i3/config").exists(),
        "i3 config should be installed"
    );
    assert!(
        base_path.join(".config/i3/i3status-retro.conf").exists(),
        "i3status config should be installed"
    );
    assert!(
        base_path.join(".config/kitty/theme.conf").exists(),
        "Kitty config should be installed"
    );
    assert!(
        base_path.join(".config/conky/retro-bbc.conkyrc").exists(),
        "Conky config should be installed"
    );
    assert!(
        base_path
            .join(".local/bin/wallpaper_downloader_final.py")
            .exists(),
        "Wallpaper downloader should be installed"
    );

    // Verify wallpaper downloader is executable
    let wallpaper_path = base_path.join(".local/bin/wallpaper_downloader_final.py");
    let metadata = std::fs::metadata(&wallpaper_path)?;
    let permissions = metadata.permissions();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert!(
            permissions.mode() & 0o111 != 0,
            "Wallpaper downloader should be executable"
        );
    }

    Ok(())
}

#[test]
fn test_dependency_checks() -> Result<()> {
    // Test command existence detection
    assert!(command_exists("ls"), "ls command should exist");
    assert!(
        !command_exists("nonexistent-command-12345"),
        "Non-existent command should return false"
    );

    // Test dependency check functions (they should not fail even if dependencies missing)
    let result = ensure_i3_installed();
    assert!(result.is_ok(), "i3 dependency check should not fail");

    let result = ensure_kitty_installed();
    assert!(result.is_ok(), "Kitty dependency check should not fail");

    let result = ensure_retro_theme_dependencies();
    assert!(
        result.is_ok(),
        "Retro theme dependency check should not fail"
    );

    Ok(())
}

#[test]
fn test_theme_file_install() -> Result<()> {
    // Test individual theme file installation
    let temp_dir = tempdir()?;
    let source = PathBuf::from("Cargo.toml");
    let target = temp_dir.path().join("test-config");

    let config = ThemeConfig {
        name: "test",
        resource_path: source.clone(),
        target_path: target.clone(),
        is_executable: false,
    };

    let result = install_theme_file(&config);
    assert!(result.is_ok(), "Theme file installation should succeed");
    assert!(target.exists(), "Target file should exist");

    // Test executable file installation
    let exec_target = temp_dir.path().join("executable-test");
    let exec_config = ThemeConfig {
        name: "exec-test",
        resource_path: source,
        target_path: exec_target.clone(),
        is_executable: true,
    };

    let result = install_theme_file(&exec_config);
    assert!(
        result.is_ok(),
        "Executable file installation should succeed"
    );
    assert!(exec_target.exists(), "Executable target file should exist");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&exec_target)?;
        let permissions = metadata.permissions();
        assert!(
            permissions.mode() & 0o111 != 0,
            "Executable file should have executable permissions"
        );
    }

    Ok(())
}

#[test]
fn test_parent_directory_creation() -> Result<()> {
    // Test that parent directories are created automatically
    let temp_dir = tempdir()?;
    let deep_path = temp_dir.path().join("deep/nested/path/config");
    let source = PathBuf::from("Cargo.toml");

    let config = ThemeConfig {
        name: "deep-test",
        resource_path: source,
        target_path: deep_path.clone(),
        is_executable: false,
    };

    let result = install_theme_file(&config);
    assert!(result.is_ok(), "Deep path installation should succeed");
    assert!(deep_path.exists(), "Deep path target should exist");

    Ok(())
}
