pub trait IWindow {
    fn process_input(&mut self);
    fn swap_buffer(&mut self);
    fn should_close(&self)->bool;
    fn handle_window_event();
}