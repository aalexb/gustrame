use crate::graphics::*;


pub struct Game{
    state:GameState,
    pub keys:[bool;1024],
    width:u32,
    height:u32,
    win:Box<dyn interface::IWindow>,
    delta_time:f64,
    last_frame:f64,
}

impl Game {
    pub fn new(width:u32, height:u32)->Self {
        let window = create_window(width,height);
        gl_load(width as i32,height as i32);
        Self { state: GameState::GameActive, 
            keys: [false;1024], width, height, 
            win:Box::new(window),
            delta_time:0.0,last_frame:0.0 }
    }

    pub fn run(&mut self) {
        while self.state==GameState::GameActive {
            let current_frame=self.win.get_time();
            self.delta_time=current_frame-self.last_frame;
            self.process_input(self.delta_time);
            self.update(self.delta_time);
            self.render();
        }
    }
    pub fn process_input(&mut self,dt:f64) {

        for action in self.win.process_input() {
            match action {
                interface::WinInteractions::Key(num, press) => self.keys[num]=press,
                interface::WinInteractions::CloseWindow => self.state=GameState::GameWin,
                interface::WinInteractions::None => {},
            }
        }

    }
    pub fn update(&mut self,dt:f64) {
        
    }
    pub fn render(&mut self) {
        gl_render();
        self.win.swap_buffer()
    }
    pub fn state(&self)->GameState {
        self.state
    }
}
#[derive(Clone,Copy,PartialEq)]
pub enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}