# EX_AA04: Roaming Profiles

**Objective**: Allow users to apply remote configurations via `mash-setup --url <url>`.

## 🛠️ Technical Implementation
1.  **CLI Argument**:
    *   Add `--url <URL>` to `installer-cli` arguments.
2.  **Fetcher**:
    *   Use the new `mash_system::curl` to download the remote TOML file to a temp location.
3.  **Loader**:
    *   Inject the downloaded config into `ConfigService` as an override.
    *   Ensure it takes precedence over local defaults but yields to CLI flags.
4.  **Security**:
    *   Print the domain of the URL and ask for explicit confirmation in TUI mode (unless `--non-interactive`).

## 🧪 Verification
*   `mash-setup --url https://example.com/my-config.toml` successfully configures the session.
