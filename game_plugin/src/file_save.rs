// rustc does not track importing macros as import
#[allow(unused_imports)]
use bevy::prelude::{info, warn};
/// rustc catchis this as a warning
#[allow(unused_variables)]
pub fn save(data: &[u8]) {
    #[cfg(target_arch = "wasm32")]
    save_web(data);
    #[cfg(feature = "native")]
    save_native(data);
}

#[cfg(target_arch = "wasm32")]
fn save_web(_data: &[u8]) {
    warn!("file save not yet supported on the web");
}
#[cfg(feature = "native")]
fn save_native(data: &[u8]) {
    use std::fs::File;
    use std::io::prelude::*;
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("numpy", &["npy"])
        .save_file()
    {
        info!("choice: {:#?}", path);
        let mut file = File::create(path).expect("failed to create file");
        file.write(data).expect("failed to write");
    }
}
