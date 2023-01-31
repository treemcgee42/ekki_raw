// S==== IMPORTS {{{1

use crate::input_state::{InputState, MouseState};
use crate::{ApplicationState, Renderer, WindowState};
use winit::event::{ElementState, WindowEvent};
use winit::event::{Event, MouseButton};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

// E==== IMPORTS }}}1

pub async fn run_event_loop(mut app_state: ApplicationState) {
    app_state
        .event_loop
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                let info = HandleWindowEventArgs {
                    event,
                    control_flow,
                    window_id,
                    window_state: &mut app_state.window_state,
                    input_state: &mut app_state.input_state,
                    renderer: &mut app_state.renderer,
                };

                handle_window_event(info);
            }

            Event::RedrawRequested(window_id) => {
                let info = HandleRedrawRequestedArgs {
                    window_id,
                    renderer: &mut app_state.renderer,
                    window_state: &mut app_state.window_state,
                    control_flow,
                };

                handle_redraw_requested(info);
            }

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                app_state.window_state.raw_window.request_redraw();
            }

            _ => {}
        });
}

// S==== Handler functions {{{1

struct HandleRedrawRequestedArgs<'a> {
    window_id: WindowId,
    renderer: &'a mut Renderer,
    window_state: &'a mut WindowState,
    control_flow: &'a mut ControlFlow,
}

fn handle_redraw_requested(info: HandleRedrawRequestedArgs) {
    if info.window_id != info.window_state.raw_window.id() {
        return;
    }

    info.renderer.update();
    match info.renderer.render() {
        Ok(_) => {}
        // Reconfigure the surface if lost
        Err(wgpu::SurfaceError::Lost) => info.renderer.resize(info.window_state.size),
        // The system is out of memory, we should probably quit
        Err(wgpu::SurfaceError::OutOfMemory) => *info.control_flow = ControlFlow::Exit,
        // All other errors (Outdated, Timeout) should be resolved by the next frame
        Err(e) => eprintln!("{:?}", e),
    }
}

struct HandleWindowEventArgs<'a> {
    event: &'a WindowEvent<'a>,
    control_flow: &'a mut ControlFlow,
    window_id: WindowId,
    window_state: &'a mut WindowState,
    input_state: &'a mut InputState,
    renderer: &'a mut Renderer,
}

fn handle_window_event(info: HandleWindowEventArgs) {
    if info.window_id != info.window_state.raw_window.id() {
        return;
    }

    // May not be necessary since we are directly handling below.:q
    if info.renderer.event_was_processed(info.event) {
        return;
    }

    match info.event {
        WindowEvent::CloseRequested => {
            *info.control_flow = ControlFlow::Exit;
        }

        WindowEvent::Resized(physical_size) => {
            println!("resizing!");
            info.window_state.size = *physical_size;
            info.renderer.resize(*physical_size);
        }

        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            info.window_state.size = **new_inner_size;
            info.renderer.resize(**new_inner_size);
        }

        WindowEvent::CursorMoved { position, .. } => {
            info.input_state.mouse_state.position = (position.x, position.y);
        }

        WindowEvent::MouseInput { state, button, .. } => {
            let info = HandleMouseInputInfo {
                mouse_state: &mut info.input_state.mouse_state,
                state,
                button,
            };
            handle_mouse_input(info);
        }

        _ => {}
    }
}

struct HandleMouseInputInfo<'a> {
    mouse_state: &'a mut MouseState,
    state: &'a ElementState,
    button: &'a MouseButton,
}

fn handle_mouse_input(info: HandleMouseInputInfo) {
    let button_pressed = *info.state == ElementState::Pressed;

    match info.button {
        MouseButton::Left => {
            info.mouse_state.button_state.left_pressed = button_pressed;
        }

        MouseButton::Right => {
            info.mouse_state.button_state.right_pressed = button_pressed;
        }

        MouseButton::Middle => {
            info.mouse_state.button_state.middle_pressed = button_pressed;
        }

        _ => {}
    }
}

// E==== Handler functions }}}1
