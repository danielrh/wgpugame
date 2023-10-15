pub mod game;
mod input;
pub mod menu;
mod render;
mod sound;
mod state;
mod system;
mod util;
mod xset;

use input::Input;
use system::System;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use xset::Xset;

use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() {
    Xset::init();
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let monitor = event_loop.primary_monitor().unwrap();
    let video_mode = monitor.video_modes().next();
    let size = video_mode
        .clone()
        .map_or(PhysicalSize::new(800, 600), |vm| vm.size());
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title("Pong")
        //        .with_fullscreen(video_mode.map(|vm| Fullscreen::Exclusive(vm)))
        .build(&event_loop)
        .unwrap();

    if window.fullscreen().is_none() {
        window.set_inner_size(PhysicalSize::new(512, 512));
    }

    window.set_cursor_visible(false);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;

                // Request fullscreen, if denied, continue as normal
                match canvas.request_fullscreen() {
                    Ok(_) => {}
                    Err(_) => (),
                }

                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    log::info!("Setup...");

    let mut render = pollster::block_on(render::Render::new(&window, size));
    let mut state = game::State::new(&render);

    log::info!("Sound...");

    let sound_system = sound::SoundSystem::new();
    let sound_pack = sound::SoundPack::new();
    let mut events = Vec::new();
    let mut input = Input::new();

    log::info!("Initializing Systems...");

    let mut menu_system = system::MenuSystem;
    let mut play_system = game::PlaySystem;
    let mut game_over_system = system::GameOverSystem::new();

    let mut visiblity_system = system::VisibilitySystem;
    visiblity_system.start(&mut state);

    menu_system.start(&mut state);

    window.set_visible(true);

    log::info!("Event Loop...");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if state.game_state == game::GameState::Quiting {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (x, y) },
                ..
            } => {
                input.mouse_move_x = x;
                input.mouse_move_y = y;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        position: position, ..
                    },
                ..
            } => {
                input.mouse_x = position.x;
                input.mouse_y = position.y;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                state.game_state = game::GameState::Quiting;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: element_state,
                        button: button,
                        ..
                    },
                ..
            } => {
                let mut index: usize;
                match button {
                    MouseButton::Left => {
                        index = 0;
                    }
                    MouseButton::Right => {
                        index = 1;
                    }
                    MouseButton::Middle => {
                        index = 2;
                    }
                    MouseButton::Other(which) => {
                        if (which as usize) < input.mouse_pressed.len() {
                            index = which as usize;
                        } else {
                            index = 1;
                        }
                    }
                }
                if input.mouse_pressed[index] && element_state != ElementState::Pressed {
                    input.mouse_click[index] = true;
                } else {
                    input.mouse_click[index] = false;
                }
                input.mouse_pressed[index] = element_state == ElementState::Pressed;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: element_state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let input_handled = match state.game_state {
                    game::GameState::Quiting => {
                        Xset::deinit();
                        true
                    }
                    _ => input.update(key, element_state),
                };
                if !input_handled {
                    process_input(element_state, key, control_flow);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                render.resize(size);
                events.push(state::Event::Resize(size.width as f32, size.height as f32));
            }
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                for event in &events {
                    match event {
                        state::Event::FocusChanged | state::Event::ButtonPressed => {
                            sound_system.queue(sound_pack.bounce());
                        }
                        state::Event::Score(_) => {
                            sound_system.queue(sound_pack.bounce());
                        }
                        state::Event::Resize(width, height) => {
                            // TODO: there should be a system that handles this
                            state.resize(*width, *height);
                            state.menu.win_text.position = (width * 0.5, height * 0.5).into();
                        }
                    }
                }
                events.clear();

                visiblity_system.update_state(&input, &mut state, &mut events);
                match state.game_state {
                    game::GameState::MainMenu => {
                        menu_system.update_state(&input, &mut state, &mut events);
                        if state.game_state == game::GameState::Playing {
                            play_system.start(&mut state);
                        }
                    }
                    game::GameState::Playing => {
                        play_system.update_state(&input, &mut state, &mut events);
                        if state.game_state == game::GameState::GameOver {
                            game_over_system.start(&mut state);
                        }
                    }
                    game::GameState::GameOver => {
                        game_over_system.update_state(&input, &mut state, &mut events);
                        if state.game_state == game::GameState::MainMenu {
                            menu_system.start(&mut state);
                        }
                    }
                    game::GameState::Quiting => {
                        Xset::deinit();
                    }
                }

                render.render_state(&state);
                if state.game_state != game::GameState::Quiting {
                    window.request_redraw();
                } else {
                    Xset::deinit();
                }
            }
            _ => {}
        }
    });
}

fn process_input(
    element_state: ElementState,
    keycode: VirtualKeyCode,
    control_flow: &mut ControlFlow,
) {
    match (keycode, element_state) {
        (VirtualKeyCode::Escape, ElementState::Pressed) => {
            Xset::deinit();
            *control_flow = ControlFlow::Exit;
        }
        _ => {}
    }
}
