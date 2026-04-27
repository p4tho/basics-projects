// main.rs

use wgpu_fundamentals::WgpuState;
use pollster::block_on;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{PhysicalKey, KeyCode},
    window::{Window},
};

struct App<'a> {
    window: Option<Window>,
    gpu: Option<WgpuState<'a>>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self { window: None, gpu: None }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("wgpu Triangle"))
            .unwrap();

        let gpu = block_on(WgpuState::new(unsafe {  // <-- wrap with block_on
            std::mem::transmute::<&Window, &'static Window>(&window)
        }));

        self.window = Some(window);
        self.gpu = Some(gpu);
    }


    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.gpu {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {
                state.update();
                state.render();
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}