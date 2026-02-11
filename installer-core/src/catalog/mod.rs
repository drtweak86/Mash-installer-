use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ProgramOption {
    pub name: &'static str,
    pub description: &'static str,
    pub default: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Category {
    pub name: &'static str,
    pub options: Vec<ProgramOption>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Catalog {
    pub categories: Vec<Category>,
}

impl Catalog {
    pub fn curated() -> Self {
        Self {
            categories: vec![
                Category {
                    name: "Shell & terminal",
                    options: vec![
                        ProgramOption {
                            name: "zsh",
                            description: "mature, scriptable, and widely supported by frameworks like Oh My Zsh and Powerlevel10k.",
                            default: true,
                        },
                        ProgramOption {
                            name: "fish",
                            description: "smart autocompletion and sane defaults when you want configuration-free modernity.",
                            default: false,
                        },
                        ProgramOption {
                            name: "bash",
                            description: "the smallest common denominator for scripting, automation, and rescue shells.",
                            default: false,
                        },
                        ProgramOption {
                            name: "pwsh",
                            description: "PowerShell Core brings cross-platform object pipelines if you need Windows-like scripts or Azure tooling.",
                            default: false,
                        },
                        ProgramOption {
                            name: "elvish",
                            description: "expressive data structures and shared history make it ideal for tinkering without losing context.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Editors",
                    options: vec![
                        ProgramOption {
                            name: "neovim",
                            description: "fast over SSH, great on Pi, huge ecosystem.",
                            default: true,
                        },
                        ProgramOption {
                            name: "micro",
                            description: "\"nano but modern,\" perfect when you just need to edit a file.",
                            default: false,
                        },
                        ProgramOption {
                            name: "helix",
                            description: "modern modal editor with few plugins and sensible defaults out of the box.",
                            default: false,
                        },
                        ProgramOption {
                            name: "vim",
                            description: "smallest common denominator, always available, perfect for minimal recoveries.",
                            default: false,
                        },
                        ProgramOption {
                            name: "code-server",
                            description: "VS Code in the browser when you want full IDE features on a phone or tablet.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Dev toolchains",
                    options: vec![
                        ProgramOption {
                            name: "rustup",
                            description: "bump-free Rust toolchain plus cargo helpers (watch, fmt, clippy) for any project.",
                            default: true,
                        },
                        ProgramOption {
                            name: "python3",
                            description: "ubiquitous interpreter for scripts, build helpers, and automation on Pi and SBCs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "nodejs",
                            description: "Web tooling, TypeScript builds, and npm-powered CLIs run natively on modern builds.",
                            default: false,
                        },
                        ProgramOption {
                            name: "build-essential / base-devel",
                            description: "barebones GCC/make chain for C/C++ projects that still ship on many boards.",
                            default: false,
                        },
                        ProgramOption {
                            name: "go",
                            description: "Go compiler is fast to install and handles small cross-platform utilities or networking daemons.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Containers",
                    options: vec![
                        ProgramOption {
                            name: "docker",
                            description: "widest ecosystem, tutorials, and muscle memory.",
                            default: true,
                        },
                        ProgramOption {
                            name: "podman",
                            description: "daemonless, rootless-friendly, Linux-native vibe with Docker-compatible CLI.",
                            default: false,
                        },
                        ProgramOption {
                            name: "nerdctl + containerd",
                            description: "lightweight stack closer to the Kubernetes/containerd world.",
                            default: false,
                        },
                        ProgramOption {
                            name: "lxc/lxd",
                            description: "system containers that feel like mini VMs, great for isolation without full virtualization.",
                            default: false,
                        },
                        ProgramOption {
                            name: "distrobox",
                            description: "not a runtime, but fantastic for disposable distro-specific dev environments inside a container.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Git & GitHub workflows",
                    options: vec![
                        ProgramOption {
                            name: "git",
                            description: "rock-solid source control that every workflow assumes is installed.",
                            default: true,
                        },
                        ProgramOption {
                            name: "gh",
                            description: "GitHub CLI for PRs, issues, and automation straight from the terminal.",
                            default: false,
                        },
                        ProgramOption {
                            name: "git-lfs",
                            description: "manage large media files without bloating the repo, especially useful for offline Pi builds.",
                            default: false,
                        },
                        ProgramOption {
                            name: "lazygit",
                            description: "TUI for staging, rebasing, and history browsing when you prefer visual helpers.",
                            default: false,
                        },
                        ProgramOption {
                            name: "forge",
                            description: "GitHub-native multiplexer for people who need 'gh' + 'git' in one assisted experience.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "System monitoring",
                    options: vec![
                        ProgramOption {
                            name: "htop",
                            description: "simple process tree with CPU/memory bars you can trust on Pi boards.",
                            default: true,
                        },
                        ProgramOption {
                            name: "btop",
                            description: "prettier stats, graphs, and widgets when you crave a dashboard feel.",
                            default: false,
                        },
                        ProgramOption {
                            name: "glances",
                            description: "cross-platform, exposes metrics via API/HTML for remote monitoring sessions.",
                            default: false,
                        },
                        ProgramOption {
                            name: "nmon",
                            description: "lightweight and scriptable, handy when zoned into troubleshooting on older SBCs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "bpytop",
                            description: "Python-based, fancier output if you want a slick, configurable console UI.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "File management",
                    options: vec![
                        ProgramOption {
                            name: "nnn",
                            description: "insanely fast navigator with optional file previews and plugin hooks.",
                            default: true,
                        },
                        ProgramOption {
                            name: "ranger",
                            description: "vim-style interface that shines when you like directory trees and multi-column views.",
                            default: false,
                        },
                        ProgramOption {
                            name: "mc",
                            description: "Midnight Commander remains familiar to admins who grew up on console file dialogs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "lf",
                            description: "minimal, lua-flagged file manager that leans heavily on shell pipeability.",
                            default: false,
                        },
                        ProgramOption {
                            name: "tmsu",
                            description: "tags instead of directories, perfect for messy media collections on SD cards.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Networking & remote",
                    options: vec![
                        ProgramOption {
                            name: "openssh",
                            description: "ubiquitous SSH client/server stack for remote shells and file copies.",
                            default: true,
                        },
                        ProgramOption {
                            name: "dropbear",
                            description: "tiny SSH implementation when you need a lightweight footprint on resource-constrained boards.",
                            default: false,
                        },
                        ProgramOption {
                            name: "mosh",
                            description: "mobile-optimized remote shells that survive flaky Wi-Fi and roaming IPs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "tmate",
                            description: "shareable SSH session for pair programming or remote debugging without complex VPNs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "sshfs",
                            description: "mount remote filesystems transparently and work on them like local files.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Backups & sync",
                    options: vec![
                        ProgramOption {
                            name: "rsync",
                            description: "battle-tested sync for SD cards, thumb drives, and over-the-wire copies.",
                            default: true,
                        },
                        ProgramOption {
                            name: "rclone",
                            description: "cloud-backed sync that speaks s3, GDrive, and many other remotes when you need offsite storage.",
                            default: false,
                        },
                        ProgramOption {
                            name: "borg",
                            description: "deduplicating backup with compression and encryption that protects repeated builds.",
                            default: false,
                        },
                        ProgramOption {
                            name: "restic",
                            description: "easy snapshots with built-in verification, ideal for immutable Pi images.",
                            default: false,
                        },
                        ProgramOption {
                            name: "duplicity",
                            description: "incremental backups over SSH/FTP when you want GPG-encrypted archives.",
                            default: false,
                        },
                    ],
                },
                Category {
                    name: "Pi/SBC hardware extras",
                    options: vec![
                        ProgramOption {
                            name: "argononed fan control (Argon One module)",
                            description: "tuned fan curves plus service integration tailored to Argon cases.",
                            default: true,
                        },
                        ProgramOption {
                            name: "pigpio",
                            description: "background daemon for PWM, hardware PWM, and DMA-driven GPIO toggling.",
                            default: false,
                        },
                        ProgramOption {
                            name: "gpiozero",
                            description: "beginner-friendly Python interface for buttons, LEDs, and sensors on a Pi.",
                            default: false,
                        },
                        ProgramOption {
                            name: "libgpiod",
                            description: "chardev-based GPIO tooling when you need modern kernel interfaces instead of sysfs.",
                            default: false,
                        },
                        ProgramOption {
                            name: "RPi.GPIO",
                            description: "classic Python library kept around for legacy scripts and teaching examples.",
                            default: false,
                        },
                    ],
                },
            ],
        }
    }
}

pub fn curated_catalog() -> Catalog {
    Catalog::curated()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_category_has_five_options() {
        let catalog = curated_catalog();
        for category in &catalog.categories {
            assert_eq!(
                category.options.len(),
                5,
                "{} has {} options",
                category.name,
                category.options.len()
            );
        }
    }

    #[test]
    fn each_category_has_one_default() {
        let catalog = curated_catalog();
        for category in &catalog.categories {
            let default_count = category.options.iter().filter(|opt| opt.default).count();
            assert_eq!(
                default_count,
                1,
                "{} has {} defaults",
                category.name,
                default_count
            );
        }
    }
}
