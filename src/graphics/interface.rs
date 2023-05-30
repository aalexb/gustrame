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