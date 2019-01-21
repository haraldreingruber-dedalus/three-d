
use glutin::*;
use dust::camerahandler::CameraHandler;
use dust::camera::Camera;

pub struct WindowHandler
{
    gl_window: GlWindow,
    events_loop: EventsLoop,
    gl: gl::Gl
}

impl WindowHandler
{
    pub fn new_default(title: &str) -> WindowHandler
    {
        let width: usize = 900;
        let height: usize = 700;
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dpi::LogicalSize::new(width as f64, height as f64));

        WindowHandler::new(window)
    }

    pub fn new(window: WindowBuilder) -> WindowHandler
    {
        let events_loop = EventsLoop::new();

        let context = ContextBuilder::new().with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }
        let gl = gl::Gl::load_with(|s| gl_window.get_proc_address(s) as *const std::os::raw::c_void);
        WindowHandler {gl_window, events_loop, gl}
    }

    pub fn size(&self) -> (usize, usize)
    {
        let size: (u32, u32) = self.gl_window.get_inner_size().unwrap().to_physical(self.gl_window.get_hidpi_factor()).into();
        (size.0 as usize, size.1 as usize)
    }

    pub fn gl(&self) -> gl::Gl
    {
        self.gl.clone()
    }

    pub fn handle_events<F>(&mut self, mut callback: F)
        where F: FnMut(&Event)
    {
        self.events_loop.poll_events(|event| {
            callback(&event);
        });
    }

    pub fn handle_camera_events(event: &Event, camera_handler: &mut CameraHandler, camera: &mut Camera)
    {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Tab && input.state == ElementState::Pressed
                        {
                            camera_handler.next_state();
                        }
                    }
                },
                WindowEvent::MouseWheel {delta, ..} => {
                    if let MouseScrollDelta::LineDelta(x,y) = delta
                    {
                        camera_handler.zoom(camera, *y as i32);
                    }
                },
                _ => ()
            },
            _ => ()
        }


    }

    pub fn handle_window_close_events(event: &Event)
    {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::CloseRequested => std::process::exit(1),
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Escape
                        {
                            std::process::exit(1);
                        }
                    }
                },
                _ => ()
            },
            _ => ()
        }
    }

    pub fn swap_buffers(&self)
    {
        self.gl_window.swap_buffers().unwrap();
    }
}