mod window;
mod shader;
mod texture;
mod mesh;
mod interface;
pub mod game;





pub fn  hello_from_lib()
{
    println!("Hello its me, lib!");
}

fn handle_window_event(window: &mut glfw::Window, event:glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _)=>{
            window.set_should_close(true)
        }
        _=>{}
    }
}



