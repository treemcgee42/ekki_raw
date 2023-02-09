// S==== IMPORTS {{{1

use crate::camera::Camera;
use crate::input_state::{ButtonPressedInfo, InputState, KeyboardState, MouseState, PressedBool};
use crate::{ApplicationState, Renderer, WindowState};
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use winit::event::{Event, MouseButton};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

// E==== IMPORTS }}}1

pub async fn run_event_loop(event_loop: EventLoop<()>, mut app_state: ApplicationState) {
    event_loop.run(move |event, _, control_flow| match event {
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
                camera: &mut app_state.camera,
            };

            handle_window_event(info);
        }

        Event::RedrawRequested(window_id) => {
            let info = HandleRedrawRequestedArgs {
                window_id,
                control_flow,
                app_state: &mut app_state,
            };

            handle_redraw_requested(info);
        }

        Event::MainEventsCleared => {
            if app_state.input_state.viewport_camera_rotate() {
                let delta_mouse = cgmath::Vector2::new(
                    (app_state.input_state.mouse_state.current_position.0
                        - app_state
                            .input_state
                            .mouse_state
                            .button_state
                            .left_button
                            .position_when_pressed
                            .unwrap()
                            .0) as f32,
                    (app_state.input_state.mouse_state.current_position.1
                        - app_state
                            .input_state
                            .mouse_state
                            .button_state
                            .left_button
                            .position_when_pressed
                            .unwrap()
                            .1) as f32,
                );

                let p_start = app_state
                    .input_state
                    .mouse_state
                    .button_state
                    .left_button
                    .position_when_pressed
                    .unwrap();
                let p_current = app_state.input_state.mouse_state.current_position;

                let window_size = (
                    app_state.window_state.size.width as f32,
                    app_state.window_state.size.height as f32,
                );

                // app_state.camera.t(
                //     p_start,
                //     p_current,
                //     (
                //         app_state.window_state.size.width as f32,
                //         app_state.window_state.size.height as f32,
                //     ),
                // );
                app_state.camera.turntable_rotate(delta_mouse, window_size);
            } else {
                // TODO: move
                app_state.camera.solidify_view_info();
            }

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
    control_flow: &'a mut ControlFlow,
    app_state: &'a mut ApplicationState,
}

fn handle_redraw_requested(info: HandleRedrawRequestedArgs) {
    if info.window_id != info.app_state.window_state.raw_window.id() {
        return;
    }

    info.app_state.renderer.update(&info.app_state.camera);
    match info.app_state.renderer.render() {
        Ok(_) => {}
        // Reconfigure the surface if lost
        Err(wgpu::SurfaceError::Lost) => info
            .app_state
            .renderer
            .resize(&mut info.app_state.camera, info.app_state.window_state.size),
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
    camera: &'a mut Camera,
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
            info.window_state.size = *physical_size;
            info.renderer.resize(info.camera, *physical_size);
        }

        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            info.window_state.size = **new_inner_size;
            info.renderer.resize(info.camera, **new_inner_size);
        }

        WindowEvent::CursorMoved { position, .. } => {
            info.input_state.mouse_state.previous_position =
                info.input_state.mouse_state.current_position;
            // TODO: abstract
            let x = f64::min(
                f64::max(position.x, 0.0),
                info.window_state.size.width as f64,
            );

            let y = f64::min(
                f64::max(position.y, 0.0),
                info.window_state.size.height as f64,
            );
            info.input_state.mouse_state.current_position = (x, y);
        }

        WindowEvent::MouseInput { state, button, .. } => {
            let info = HandleMouseInputInfo {
                mouse_state: &mut info.input_state.mouse_state,
                state,
                button,
            };
            handle_mouse_input(info);
        }

        WindowEvent::KeyboardInput { input, .. } => {
            let args = HandleKeyboardInputArgs {
                keyboard_state: &mut info.input_state.keyboard_state,
                state: &input.state,
                key: &input.virtual_keycode,
            };
            handle_keyboard_input(args);
        }

        _ => {}
    }
}

struct HandleKeyboardInputArgs<'a> {
    keyboard_state: &'a mut KeyboardState,
    state: &'a ElementState,
    key: &'a Option<VirtualKeyCode>,
}

fn handle_keyboard_input(args: HandleKeyboardInputArgs) {
    if args.key.is_none() {
        return;
    }

    let pressed = PressedBool {
        is_pressed: *args.state == ElementState::Pressed,
    };

    match args.key.unwrap() {
        VirtualKeyCode::LAlt => {
            args.keyboard_state.option_alt_key = pressed;
        }

        VirtualKeyCode::RAlt => {
            args.keyboard_state.option_alt_key = pressed;
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
            let pos = {
                if button_pressed {
                    Some(info.mouse_state.current_position)
                } else {
                    None
                }
            };

            info.mouse_state.button_state.left_button = ButtonPressedInfo {
                is_pressed: button_pressed,
                position_when_pressed: pos,
            };
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
