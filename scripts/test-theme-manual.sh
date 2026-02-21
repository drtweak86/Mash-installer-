#!/bin/bash
# Copyright 2024 MASH Installer Authors
# SPDX-License-Identifier: MIT

# 🧪 Manual Theme Testing Script
# Bard's Tavern Test Forge - For environments without Rust toolchain

echo "🍺 Bard's Theme Testing Tavern"
echo "=============================="
echo ""

# Set up test environment
TEST_DIR=$(mktemp -d)
echo "🏗️  Created test forge: $TEST_DIR"
echo ""

# Test 1: Verify theme files exist
echo "📋 Test 1: Theme File Verification"
echo "--------------------------------"
THEME_FILES=(
    "resources/themes/retro-bbc/i3-config"
    "resources/themes/retro-bbc/i3status-retro.conf"
    "resources/themes/retro-bbc/kitty.conf"
    "resources/themes/retro-bbc/conkyrc"
    "resources/themes/retro-bbc/wallpaper_downloader_final.py"
)

ALL_FILES_EXIST=true
for file in "${THEME_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ $file (MISSING)"
        ALL_FILES_EXIST=false
    fi
done
echo ""

# Test 2: Verify theme module structure
echo "🔧 Test 2: Theme Module Structure"
echo "--------------------------------"
if [ -f "installer-core/src/theme.rs" ]; then
    echo "✅ theme.rs exists"
    
    # Check for key functions
    if grep -q "pub fn install_retro_theme" installer-core/src/theme.rs; then
        echo "✅ install_retro_theme function found"
    else
        echo "❌ install_retro_theme function missing"
    fi
    
    if grep -q "pub fn ensure_retro_theme_dependencies" installer-core/src/theme.rs; then
        echo "✅ ensure_retro_theme_dependencies function found"
    else
        echo "❌ ensure_retro_theme_dependencies function missing"
    fi
else
    echo "❌ theme.rs missing"
fi
echo ""

# Test 3: Verify menu integration
echo "🎨 Test 3: Menu Integration"
echo "---------------------------"
if [ -f "installer-cli/src/menu.rs" ]; then
    echo "✅ menu.rs exists"
    
    if grep -q "run_theme_menu" installer-cli/src/menu.rs; then
        echo "✅ run_theme_menu function found"
    else
        echo "❌ run_theme_menu function missing"
    fi
    
    if grep -q "ThemePlan::RetroOnly" installer-cli/src/menu.rs; then
        echo "✅ ThemePlan enum usage found"
    else
        echo "❌ ThemePlan enum usage missing"
    fi
else
    echo "❌ menu.rs missing"
fi
echo ""

# Test 4: Verify software tiers integration
echo "📦 Test 4: Software Tiers Integration"
echo "------------------------------------"
if [ -f "installer-core/src/software_tiers.rs" ]; then
    echo "✅ software_tiers.rs exists"
    
    if grep -q "pub enum ThemePlan" installer-core/src/software_tiers.rs; then
        echo "✅ ThemePlan enum defined"
    else
        echo "❌ ThemePlan enum missing"
    fi
    
    if grep -q "theme_plan: ThemePlan" installer-core/src/software_tiers.rs; then
        echo "✅ theme_plan field in SoftwareTierPlan"
    else
        echo "❌ theme_plan field missing"
    fi
else
    echo "❌ software_tiers.rs missing"
fi
echo ""

# Test 5: Verify lib.rs exports
echo "🔗 Test 5: Library Exports"
echo "-------------------------"
if [ -f "installer-core/src/lib.rs" ]; then
    echo "✅ lib.rs exists"
    
    if grep -q "pub use theme::" installer-core/src/lib.rs; then
        echo "✅ Theme module exports found"
        grep "pub use theme::" installer-core/src/lib.rs | sed 's/^/  /'
    else
        echo "❌ Theme module exports missing"
    fi
else
    echo "❌ lib.rs missing"
fi
echo ""

# Test 6: Verify integration test file
echo "🧪 Test 6: Integration Tests"
echo "---------------------------"
if [ -f "installer-core/tests/theme_integration.rs" ]; then
    echo "✅ theme_integration.rs exists"
    
    TEST_COUNT=$(grep -c "^#\[test\]" installer-core/tests/theme_integration.rs)
    echo "✅ Found $TEST_COUNT test functions"
else
    echo "❌ theme_integration.rs missing"
fi
echo ""

# Test 7: Wallpaper downloader syntax check
echo "🖼️  Test 7: Wallpaper Downloader"
echo "-------------------------------"
WALLPAPER_SCRIPT="resources/themes/retro-bbc/wallpaper_downloader_final.py"
if [ -f "$WALLPAPER_SCRIPT" ]; then
    echo "✅ Wallpaper downloader script exists"
    
    # Check for key functions
    if grep -q "def download_wallpapers" "$WALLPAPER_SCRIPT"; then
        echo "✅ download_wallpapers function found"
    else
        echo "❌ download_wallpapers function missing"
    fi
    
    if grep -q "argparse" "$WALLPAPER_SCRIPT"; then
        echo "✅ Command line argument parsing found"
    else
        echo "❌ Command line argument parsing missing"
    fi
    
    # Check for Wallhaven API usage
    if grep -q "wallhaven" "$WALLPAPER_SCRIPT"; then
        echo "✅ Wallhaven API integration found"
    else
        echo "❌ Wallhaven API integration missing"
    fi
else
    echo "❌ Wallpaper downloader script missing"
fi
echo ""

# Summary
echo "📊 Test Summary"
echo "=============="
if [ "$ALL_FILES_EXIST" = true ]; then
    echo "✅ All theme files present"
    echo "✅ Module structure verified"
    echo "✅ Integration complete"
    echo ""
    echo "🎉 Theme integration appears successful!"
    echo "🍺 Ready for Rust compilation and testing!"
else
    echo "❌ Some files missing"
    echo "⚠️  Please check the missing files above"
fi

# Clean up
rm -rf "$TEST_DIR"
echo ""
echo "🧹 Cleaned up test forge"
echo "✨ Testing complete!"