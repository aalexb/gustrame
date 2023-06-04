use std::rc::Rc;

use super::texture::Texture2D;

pub trait IWindow {
    fn process_input(&mut self)->Vec<WinInteractions>;
    fn swap_buffer(&mut self);
    fn should_close(&self)->bool;
    fn get_time(&self)->f64;
}

pub enum WinInteractions {
    Key(usize,bool),
    WinSize(u32,u32),
    CloseWindow,
    None,
}

pub trait ITextureCache{
    fn get_texture(&self,name:&str)->Rc<Texture2D>;
}