use breakout::{game::Game, window::Window};
use glfw::ffi::OPENGL_CORE_PROFILE;


const WIDTH:u32=1280;
const HEIGHT:u32=720;

fn main() {
    let mut game = Game::new(WIDTH,HEIGHT);

    unsafe{
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    game.init();

    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    game.run();
    while !window.should_close() {
        let current_frame = glfw.get_time();
        delta_time = current_frame-last_frame;
        last_frame = current_frame;
        glfw.poll_events();
        for (_,event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event,&mut game);
        }
        game.process_input(delta_time);
        game.update(delta_time);
        unsafe{
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        game.render();
    }
}

fn handle_window_event(window: &mut glfw::Window, event:glfw::WindowEvent,game:&mut Game) {
    match event {
        glfw::WindowEvent::Key(key, _, action, _)=>{
            if key==glfw::Key::Escape&&action==glfw::Action::Press {
                window.set_should_close(true)
            }
            if  (key as usize)>=0&&(key as usize)<1024{
                if action==glfw::Action::Press {
                    game.keys[key as usize]=true;
                } else if action==glfw::Action::Release {
                    game.keys[key as usize]=false;
                }
            }
            
        },
        glfw::WindowEvent::FramebufferSize(w, h)=>{
            framebuffer_size_callback(window, w, h);
        }
        _=>{}
    }
}

fn framebuffer_size_callback(window:&glfw::Window, width:i32, height:i32) {
    unsafe{
        gl::Viewport(0, 0, width, height);
    }    
}