use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::Size,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::wgpu_state::State;

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_inner_size(Size::Physical(winit::dpi::PhysicalSize {
                            width: 800,
                            height: 800,
                        }))
                        .with_title("WGPU")
                        .with_resizable(false),
                )
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {
                state.render();
                state.get_window().request_redraw();
            }

            _ => (),
        }
    }
}
