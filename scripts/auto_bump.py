import argparse
import os
import re
import subprocess
from typing import List

def get_workspace_members() -> List[str]:
    """Reads Cargo.toml to find workspace members."""
    cargo_toml_path = os.path.join(os.getcwd(), 'Cargo.toml')
    if not os.path.exists(cargo_toml_path):
        raise FileNotFoundError(f"Cargo.toml not found at {cargo_toml_path}")

    with open(cargo_toml_path, 'r') as f:
        content = f.read()

    members_match = re.search(r'\[workspace\]\nmembers = \[(.*?)\]', content, re.DOTALL)
    if not members_match:
        # Fallback for single-package Cargo.toml or different format
        return []
    
    # Extract members, clean up whitespace and quotes
    members = [m.strip().strip('"') for m in members_match.group(1).split(',') if m.strip()]
    
    # Filter for actual directories relative to the workspace root
    return [m for m in members if os.path.isdir(m)]

def find_version_files(workspace_members: List[str]) -> List[str]:
    """Finds all Cargo.toml files in the workspace and relevant UI/doc files."""
    files = []
    # Cargo.toml files
    files.append('Cargo.toml') # Root Cargo.toml
    for member in workspace_members:
        files.append(os.path.join(member, 'Cargo.toml'))
    
    # UI/Doc files (hardcoded paths as they are specific to this project's structure)
    files.append('docs/MANUAL.md')
    files.append('installer-cli/src/tui/render.rs')
    files.append('installer-cli/src/tui/menus.rs')
    files.append('docs/HISTORY.md')
    files.append('rust-toolchain.toml') # Added for checking purposes, though version is not directly in it

    # Filter out files that don't exist to prevent errors
    return [f for f in files if os.path.exists(f)]

def get_current_version(file_path: str) -> str:
    """Extracts the version from a Cargo.toml or rust-toolchain.toml file."""
    with open(file_path, 'r') as f:
        content = f.read()
        match_cargo = re.search(r'version\s*=\s*"(\d+\.\d+\.\d+)"', content)
        if match_cargo:
            return match_cargo.group(1)
        
        # Check for rust-toolchain.toml channel (if needed, but usually not version-bumped)
        match_toolchain = re.search(r'channel\s*=\s*"(\d+\.\d+\.\d+)"', content)
        if match_toolchain:
            # This is typically rust version, not product version. Handle with care.
            # For this script's purpose, we'll primarily rely on Cargo.toml for product version.
            pass

    raise ValueError(f"Version not found in {file_path}")

def bump_version(current_version: str, bump_type: str) -> str:
    """Bumps the version string based on type (patch, minor, major)."""
    parts = list(map(int, current_version.split('.')))
    if bump_type == 'major':
        parts[0] += 1
        parts[1] = 0
        parts[2] = 0
    elif bump_type == 'minor':
        parts[1] += 1
        parts[2] = 0
    elif bump_type == 'patch':
        parts[2] += 1
    else:
        raise ValueError(f"Invalid bump type: {bump_type}")
    return ".".join(map(str, parts))

def update_file_version(file_path: str, old_version: str, new_version: str):
    """Updates all occurrences of the version string in a file."""
    with open(file_path, 'r') as f:
        content = f.read()

    new_content = content
    
    # Generic version string replacement (e.g., in Cargo.toml version = "X.Y.Z")
    new_content = re.sub(
        r'(version\s*=\s*")' + re.escape(old_version) + r'(")',
        r'\g<1>' + new_version + r'\g<2>',
        new_content
    )
    # Specific UI/doc replacements (e.g., "vX.Y.Z", "MASH INSTALLER vX.Y.Z", "OS REV X.Y.Z")
    # This catches versions in comments, strings, etc.
    new_content = re.sub(
        re.escape(old_version), 
        new_version, 
        new_content
    )
    # For rust-toolchain.toml, if it contains the version directly, though we don't expect it to be auto-bumped this way
    new_content = re.sub(
        r'(channel\s*=\s*")' + re.escape(old_version) + r'(")',
        r'\g<1>' + new_version + r'\g<2>',
        new_content
    )

    if new_content != content:
        print(f"Updating {file_path}: {old_version} -> {new_version}")
        with open(file_path, 'w') as f:
            f.write(new_content)
    else:
        print(f"No change needed in {file_path}")

def run_command(command: str):
    """Runs a shell command and checks for errors."""
    result = subprocess.run(command, shell=True, check=True, capture_output=True, text=True)
    if result.stdout:
        print(result.stdout)
    if result.stderr:
        print(result.stderr)

def main():
    parser = argparse.ArgumentParser(description="Automate version bumping for the MASH Installer workspace.")
    parser.add_argument('bump_type', choices=['patch', 'minor', 'major'],
                        help="Type of version bump to perform.")
    args = parser.parse_args()

    # Get current version from root Cargo.toml
    root_cargo_toml = 'Cargo.toml'
    current_version = get_current_version(root_cargo_toml)
    new_version = bump_version(current_version, args.bump_type)

    print(f"Bumping version from {current_version} to {new_version} ({args.bump_type} bump)")

    workspace_members = get_workspace_members()
    version_files = find_version_files(workspace_members)

    for file_path in version_files:
        update_file_version(file_path, current_version, new_version)
    
    print("\nRunning `cargo update` to update Cargo.lock...")
    run_command("cargo update")
    print("Running `cargo build --workspace` to validate changes and update Cargo.lock if necessary...")
    run_command("cargo build --workspace")
    print("Running `cargo test --workspace` to ensure all tests pass...")
    run_command("cargo test --workspace")

    print(f"\nVersion successfully bumped to {new_version}.")
    print("Remember to update HISTORY.md manually for new entry if this is a release, and commit changes.")

if __name__ == '__main__':
    main()
