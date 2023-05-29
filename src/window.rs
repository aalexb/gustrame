use glfw::{Context, Glfw};

use crate::interface::IWindow;

pub struct Window{
    window:glfw::Window,
    glfw:Glfw,
    events:std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {

    pub fn init(width:u32, height:u32)->Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        let (mut window, events) = glfw
        .create_window(
            width, 
            height, 
            "Breakout", 
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window"); 
        window.set_key_polling(true);
        glfw::Context::make_current(&mut window); 
        gl::load_with(|s|window.get_proc_address(s) as *const _);

        Self { window, glfw,events }      
    }
}

impl IWindow for Window {
    

    fn swap_buffer(&mut self) {
        self.window.swap_buffers();
    }


    fn process_input(&mut self) {
        self.glfw.poll_events();
        for (_,event) in glfw::flush_messages(&self.events) {
            handle_window_event(&self.window, event,&mut game);
        }
    }

    fn should_close(&self)->bool {
        self.window.should_close()
    }
       
}