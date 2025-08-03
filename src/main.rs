mod vector;
pub mod view_matrix;
use once_cell::sync::Lazy;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use memflow::prelude::v1::*;

static SCREEN_WIDTH: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CXSCREEN) as f32 }
});

static SCREEN_HEIGHT: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CYSCREEN) as f32 }
});

fn main() {

    // https://github.com/a2x/cs2-dumper/blob/main/src/main.rs
    let inventory = Inventory::scan();

    let os = {

        #[cfg(windows)]
        {
            memflow_native::create_os(&OsArgs::default(), LibArc::default())
                .unwrap_or_else(|err| {
                    eprintln!("failed to create os : {}", err);
                    std::process::exit(1);
                })
        }
        #[cfg(not(windows))]
        {
            panic!("only windows is supported")
        }
    };

}


