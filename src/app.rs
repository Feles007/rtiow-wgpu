use crate::setup;
use crate::state::State;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlMap {
	pub zoom_in: bool,
	pub zoom_out: bool,
	pub move_yaw: f32,
	pub move_pitch: f32,
	pub move_forward: bool,
	pub move_backward: bool,
	pub move_left: bool,
	pub move_right: bool,
}
pub enum App {
	Initializing,
	Running {
		state: State,
		control_map: ControlMap,
		delta_time: f32,
	},
}
impl App {
	pub fn new() -> Self {
		Self::Initializing
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

		let (world, camera_parameters) = setup();
		let state = pollster::block_on(State::new(window.clone(), camera_parameters, &world));

		window.request_redraw();
		*self = Self::Running {
			state,
			control_map: Default::default(),
			delta_time: 0.0,
		};
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		let Self::Running {
			state,
			control_map,
			delta_time,
		} = self
		else {
			return;
		};
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			},
			WindowEvent::Moved { .. } => {
				*control_map = ControlMap::default();
			},
			WindowEvent::Focused(f) => {
				if f {
					state.focus()
				} else {
					state.unfocus()
				}
			},
			WindowEvent::MouseInput {
				state: ElementState::Pressed,
				button: MouseButton::Left,
				..
			} => {
				state.focus();
			},
			WindowEvent::KeyboardInput { event, .. } => {
				let (code, pressed) = {
					let (key, pressed) = match event {
						KeyEvent {
							physical_key,
							state: ElementState::Pressed,
							..
						} => (physical_key, true),
						KeyEvent {
							physical_key,
							state: ElementState::Released,
							..
						} => (physical_key, false),
					};
					match key {
						PhysicalKey::Code(code) => (code, pressed),
						_ => return,
					}
				};

				*match code {
					KeyCode::KeyZ => &mut control_map.zoom_in,
					KeyCode::KeyX => &mut control_map.zoom_out,

					KeyCode::KeyW => &mut control_map.move_forward,
					KeyCode::KeyS => &mut control_map.move_backward,
					KeyCode::KeyA => &mut control_map.move_left,
					KeyCode::KeyD => &mut control_map.move_right,

					KeyCode::Escape => {
						state.unfocus();
						return;
					},

					_ => return,
				} = pressed;
			},
			WindowEvent::RedrawRequested => {
				let start = Instant::now();
				state.update(control_map, *delta_time);
				state.render();

				state.request_redraw();

				let elapsed = start.elapsed();
				*delta_time = elapsed.as_secs_f32();
				println!("Frame time: {:?}", elapsed);
			},
			WindowEvent::Resized(size) => {
				state.resize(size);
			},
			_ => (),
		}
	}
	fn device_event(&mut self, _: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
		let Self::Running { state, control_map, .. } = self else {
			return;
		};

		if !state.is_mouse_focused() {
			return;
		}

		match event {
			DeviceEvent::MouseMotion { delta } => {
				control_map.move_yaw += delta.0 as f32;
				control_map.move_pitch += delta.1 as f32;
			},
			_ => {},
		}
	}
}
