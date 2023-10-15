use std::process::Command;

pub struct Xset {
    _res: std::io::Result<std::process::Child>,
}

#[cfg(target_arch = "wasm32")]
impl Xset {
    pub fn new() -> Xset {
        Xset {
            _res: std::io::Error::new(std::io::ErrorKind::Unsupported, "No xset in wasm"),
        }
    }
    pub fn init() {}
    pub fn deinit() {}
}

#[cfg(not(target_arch = "wasm32"))]
impl Xset {
    fn new() -> Xset {
        Xset {
            _res: Command::new("/usr/bin/xset").arg("r").arg("off").spawn(),
        }
    }
    pub fn init() {
        XSET.get_or_init(|| Xset::new());
    }
    pub fn deinit() {
        let _ = Command::new("/usr/bin/xset").arg("r").arg("on").spawn();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Xset {
    fn drop(&mut self) {
        Xset::deinit();
    }
}
static XSET: std::sync::OnceLock<Xset> = std::sync::OnceLock::new();
