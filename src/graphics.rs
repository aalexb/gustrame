pub mod interface;
pub mod sprite;

mod buffer;
mod mesh;
mod window;
pub mod texture;
pub mod shader;

pub fn create_window(width:u32,height:u32)->impl interface::IWindow {
    window::GlfwWindow::init(width, height)
}

pub fn gl_render() {    
    pre_render();

}

fn pre_render() {
    unsafe{
        gl::ClearColor(0.2, 0.2, 0.3, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
pub fn gl_load(width:i32, height:i32) {
    unsafe{
        gl::Viewport(0, 0, width, height);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    } 
}