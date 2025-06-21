use crate::setup;
use crate::state::State;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub enum App {
	Initializing,
	Running { state: State },
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
		*self = Self::Running { state };
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		let Self::Running { state } = self else {
			return;
		};
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			},
			WindowEvent::RedrawRequested => {
				let start = Instant::now();
				state.update();
				state.render();
				let elapsed = start.elapsed();

				state.set_delta(elapsed.as_secs_f32());

				state.get_window().request_redraw();
			},
			WindowEvent::Resized(size) => {
				state.resize(size);
			},
			_ => (),
		}
	}
}
