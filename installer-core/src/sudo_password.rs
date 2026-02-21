use std::sync::{Arc, Mutex};

/// Global sudo password storage
/// This is used to temporarily store the sudo password entered by the user
/// and make it available to sudo commands
<<<<<<< HEAD
static SUDO_PASSWORD: once_cell::sync::OnceCell<Arc<Mutex<Option<String>>>> = 
=======
static SUDO_PASSWORD: once_cell::sync::OnceCell<Arc<Mutex<Option<String>>>> =
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
    once_cell::sync::OnceCell::new();

/// Initialize the sudo password storage
pub fn init_sudo_password() {
    let _ = SUDO_PASSWORD.set(Arc::new(Mutex::new(None)));
}

/// Set the sudo password
<<<<<<< HEAD
=======
#[allow(dead_code)]
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
pub fn set_sudo_password(password: String) {
    if let Some(pass_ref) = SUDO_PASSWORD.get() {
        let mut pass = pass_ref.lock().unwrap();
        *pass = Some(password);
    }
}

/// Get the sudo password
<<<<<<< HEAD
=======
#[allow(dead_code)]
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
pub fn get_sudo_password() -> Option<String> {
    if let Some(pass_ref) = SUDO_PASSWORD.get() {
        let pass = pass_ref.lock().unwrap();
        pass.clone()
    } else {
        None
    }
}

/// Clear the sudo password
<<<<<<< HEAD
=======
#[allow(dead_code)]
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
pub fn clear_sudo_password() {
    if let Some(pass_ref) = SUDO_PASSWORD.get() {
        let mut pass = pass_ref.lock().unwrap();
        *pass = None;
    }
}
<<<<<<< HEAD

/// Check if sudo password is available
pub fn has_sudo_password() -> bool {
    if let Some(pass_ref) = SUDO_PASSWORD.get() {
        let pass = pass_ref.lock().unwrap();
        pass.is_some()
    } else {
        false
    }
}
=======
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
