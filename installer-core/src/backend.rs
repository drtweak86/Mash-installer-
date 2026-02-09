#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgBackend {
    Apt,
    Pacman,
}
