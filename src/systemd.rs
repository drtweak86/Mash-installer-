pub fn is_available() -> bool {
    which::which("systemctl").is_ok()
}
