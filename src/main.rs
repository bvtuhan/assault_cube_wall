mod vector;
pub mod view_matrix;
pub mod offsets;
use debug_print::debug_println;
use once_cell::sync::Lazy;

use pixels_primitives::rect;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use memflow::prelude::v1::*;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::offsets::{entity, module_base};
use crate::vector::{Vec2, Vec3};
use crate::view_matrix::ViewMatrix;

static SCREEN_WIDTH: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CXSCREEN) as f32 }
});

static SCREEN_HEIGHT: Lazy<f32> = Lazy::new(|| {
    unsafe { GetSystemMetrics(SM_CYSCREEN) as f32 }
});

static BOX_SIZE: i16 = 64;

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
            .with_transparent(true)
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

            Event::WindowEvent { window_id: _, event } => {

                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    },

                    WindowEvent::Resized(size) => {
                        if let Err(err) = pixels.resize_surface(size.width, size.height) {
                            eprintln!("Failed to resize pixels: {}", err);
                            elwt.exit();
                        }
                    },

                    WindowEvent::RedrawRequested => {
                        let view_matrix = match process.read::<ViewMatrix>((offsets::VIEW_MATRIX_POINTER).into()) {
                            Ok(vm) => vm,
                            Err(err) => {
                                eprintln!("Failed to read view matrix: {}", err);
                                return;
                            }
                        };

                        debug_println!("View Matrix: {:#?}", view_matrix);

                        // here comes the rendering logic
                        let entity_list_ptr  = match process.read::<usize>((module_base_address + offsets::module_base::ENTITY_LIST_POINTER_OFFSET).into()) {
                            Ok(ptr) => ptr,
                            Err(err) => {
                                eprintln!("Failed to read entity list pointer: {}", err);
                                return;
                            }
                        };

                        debug_println!("Entity list pointer: {:#X}", entity_list_ptr);

                        pixels.clear_color(pixels::wgpu::Color::TRANSPARENT);

                        {
                            let frame = pixels.frame_mut();

                            for i in 0..32 {
                                let entity_base_pointer = match process.read::<usize>((entity_list_ptr + (i * 4)).into()) {
                                    Ok(ptr) => ptr,
                                    Err(err) => {
                                        eprintln!("Failed to read entity base pointer: {}", err);
                                        return;
                                    }
                                };
                                debug_println!("Entity {} base pointer: {:#X}", i, entity_base_pointer);

                                let entity_health = match process.read::<i32>((entity_base_pointer + offsets::entity::HEALTH_OFFSET).into()) {
                                    Ok(health) => health,
                                    Err(err) => {
                                        eprintln!("Failed to read entity health: {}", err);
                                        continue;
                                    }
                                };

                                if entity_health <= 0 || entity_health > 100 {
                                    continue; // skip dead or invalid entities
                                }

                                debug_println!("Entity {} health: {}", i, entity_health);

                                let mut screen = Vec2::default();
                                
                                let mut head_bone_pos : Vec3 = match process.read((entity_base_pointer + offsets::entity::HEAD_ENT_COORDINATES_OFFSET).into()) {
                                    Ok(pos) => pos,
                                    Err(err) => {
                                        eprintln!("Failed to read head coordinates: {}", err);
                                        continue;
                                    }
                                };

                                debug_println!("Entity {} head coordinates: {:?}", i, head_bone_pos);
                                
                                if !view_matrix.world_to_screen(&head_bone_pos, &mut screen) {
                                    debug_println!("Failed to convert world coordinates to screen for entity {}", i);
                                    continue;
                                }

                                debug_println!("Entity {} screen coordinates: {:?}", i, screen);
                                
                                let x = screen.x;
                                let y = screen.y;
                                let left = x as i32 - 24 / 2;
                                let right = x as i32 + 24 / 2;
                                let top = y as i32 - 16 / 2;
                                let bottom = y as i32;
                                let color = [255, 0, 0, 255]; // Red RGBA

                                rect(frame, *SCREEN_WIDTH as i32, left, top, right, bottom, &color);
                            }
                        } //frame borrow ends here

                        if let Err(err) = pixels.render() {
                            eprintln!("Failed to render pixels: {}", err);
                            elwt.exit();
                        }
                    },

                    _ => (),
                }
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}


