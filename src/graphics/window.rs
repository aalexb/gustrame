use glfw::*;
use super::interface::*;

pub struct GlfwWindow{
    window:glfw::Window,
    glfw:Glfw,
    events:std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}
impl GlfwWindow {
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

    fn handle_events(&self,event:WindowEvent)->WinInteractions {
        match event {
            glfw::WindowEvent::Key(key, _, action, _)=>{
                if key==glfw::Key::Escape&&action==glfw::Action::Press {
                    return WinInteractions::CloseWindow
                }
                if  (key as usize)>=0&&(key as usize)<1024{
                    if action==glfw::Action::Press {
                        return WinInteractions::Key(key as usize, true)
                    } else if action==glfw::Action::Release {
                        return WinInteractions::Key(key as usize, false)
                    }                    
                }
                WinInteractions::None            
            },
            glfw::WindowEvent::Close=>{
                return  WinInteractions::CloseWindow;
            }
            glfw::WindowEvent::FramebufferSize(w, h)=>{
                framebuffer_size_callback( w, h);
                WinInteractions::WinSize(w as u32, h as u32)
            },
            _=>{WinInteractions::None}
        }
    }
}
impl IWindow for GlfwWindow {
    fn swap_buffer(&mut self) {
        self.window.swap_buffers();
    }
    fn process_input(&mut self)->Vec<WinInteractions> {
        let mut que:Vec<WinInteractions>=Vec::new();
        self.glfw.poll_events();
        for (_,event) in glfw::flush_messages(&self.events) {
            que.push(self.handle_events(event));
        }
        return que
    }
    fn should_close(&self)->bool {
        self.window.should_close()
    }

    fn get_time(&self)->f64 {
        self.glfw.get_time()
    }       
}

fn framebuffer_size_callback(width:i32, height:i32) {
    unsafe{
        gl::Viewport(0, 0, width, height);
    }    
}