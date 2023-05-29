use crate::{interface::IWindow, window::Window};

pub struct Game{
    state:GameState,
    pub keys:[bool;1024],
    width:u32,
    height:u32,
    win:Box<dyn IWindow>,
    delta_time:f64,
}

impl Game {
    pub fn new(width:u32, height:u32)->Self {
        let window = Window::init(width,height);



        Self { state: GameState::GameActive, 
            keys: [false;1024], width, height, 
            win:Box::new(window),
            delta_time:0.0 }
    }
    pub fn init(&self) {
    }
    pub fn run(&mut self) {
        while !self.win.should_close() {
            self.process_input(self.delta_time);
            self.update(self.delta_time);
            self.render();
        }
    }
    pub fn process_input(&mut self,dt:f64) {
        self.win.process_input();
    }
    pub fn update(&mut self,dt:f64) {
        
    }
    pub fn render(&mut self) {

        self.win.swap_buffer()
    }
    pub fn state(&self)->GameState {
        self.state
    }
}
#[derive(Clone,Copy)]
pub enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}