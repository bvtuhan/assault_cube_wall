mod vector;
pub mod view_matrix;
pub mod offsets;
use debug_print::debug_println;
use once_cell::sync::Lazy;

use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use memflow::prelude::v1::*;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

static SCREEN_WIDTH: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CXSCREEN) as f32 }
});

static SCREEN_HEIGHT: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CYSCREEN) as f32 }
});

fn main() {

    let inventory = Inventory::scan();

    let mut os = {

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

    let mut process = os.process_by_name("ac_client.exe")
        .unwrap_or_else(|err| {
            eprintln!("failed to find process: {}", err);
            std::process::exit(1);
        });

    let event_loop = EventLoop::new().unwrap_or_else(|err|{
        eprintln!("failed to create event loop: {}", err);
        std::process::exit(1);
    });
    
    let mut input = WinitInputHelper::new();
    
    let window = {
        let size = LogicalSize::new(*SCREEN_WIDTH as f64, *SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(true)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width,
             window_size.height, &window);
        Pixels::new(*SCREEN_WIDTH as u32, *SCREEN_HEIGHT as u32, surface_texture)
            .unwrap_or_else(|err| {
                eprintln!("failed to create pixels: {}", err);
                std::process::exit(1);
            })
    };

    let module = process.module_by_name("ac_client.exe")
        .unwrap_or_else(|err| {
            eprintln!("failed to find module ac_client.exe: {}", err);
            std::process::exit(1);
        });

    debug_println!("Found module: {}", module.name);
    debug_println!("Module base address: {:#X}", module.base);
    debug_println!("Module size: {:#X}", module.size);
    debug_println!("Module path: {}", module.path);
    debug_println!("Module address: {:#X}", module.address);

    let module_base_address = module.base;

    let _ = event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        if let Err(err) = pixels.resize_surface(new_size.width, new_size.height) {
                            eprintln!("Failed to resize surface: {}", err);
                            elwt.exit();
                        }
                        debug_println!("Window resized to: {:?}", new_size);
                    },
                    WindowEvent::RedrawRequested => {
                        // here comes the rendering logic
                    },
                    _ => (),
                }
            },
            Event::MemoryWarning => eprintln!("Memory warning received"),
            _ => (),
        }
    });
}


