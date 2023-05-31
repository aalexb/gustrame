use std::rc::Rc;

use glam::{Vec2, vec3, Vec3};

use crate::graphics::{*, resmanager::ResManager, sprite::SpriteRenderer};

pub struct Game{
    state:GameState,
    pub keys:[bool;1024],
    width:u32,
    height:u32,
    win:Box<dyn interface::IWindow>,
    delta_time:f64,
    last_frame:f64,
    resmgr:ResManager,
    renderer:SpriteRenderer,
}

impl Game{
    pub fn new(width:u32, height:u32)->Self {
        let window = create_window(width,height);
        gl_load(width as i32,height as i32);

        let mut resmgr = ResManager::new();

        resmgr.LoadShader("shader.vert", "shader.frag", "sprite");
        resmgr.LoadShader("simpleshader.vert", "simpleshader.frag", "simple");
        
        
        resmgr.LoadTexture("awesomeface.png", true, "face");
        let mut renderer = SpriteRenderer::new(resmgr.GetShader("sprite"));
        renderer.initRenderData();

        Self { state: GameState::GameActive, 
            keys: [false;1024], width, height, 
            win:Box::new(window),
            delta_time:0.0,last_frame:0.0,resmgr,renderer}
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
                interface::WinInteractions::WinSize(w, h) => {
                    self.width=w;
                    self.height=h;
                }
                interface::WinInteractions::None => {},
            }
        }
    }
    pub fn update(&mut self,dt:f64) {
        
    }
    pub fn render(&mut self) {

        gl_render();
        let tupovec=Vec3::new(0.8,0.0,0.0);
        let trans = vec3(400.0,300.0,0.0);

        let projection = glam::Mat4::orthographic_rh_gl(0.0, self.width as f32, 0.0, self.height as f32, -1.0, 1.0);
        let model=glam::Mat4::IDENTITY*glam::Mat4::from_translation(trans);
        unsafe{
            
            self.resmgr.GetShader("simple").apply().SetMatrix4("projection", &projection);
            self.resmgr.GetShader("simple").SetMatrix4("model", &model);
            self.resmgr.GetShader("simple").SetVector3f("tupovec", tupovec);
        }
        
        draw_circle();                
        
        unsafe{
            self.resmgr.GetShader("sprite").apply().SetInteger("image", 0);
            self.resmgr.GetShader("sprite").SetMatrix4("projection", &projection);            
        }

        self.renderer.DrawSprite(
            &self.resmgr.GetShader("sprite"),
            self.resmgr.GetTexture("face"), 
            Vec2::new(200.0, 200.0), 
            Vec2::new(300.0, 400.0), 
            45.0, 
            vec3(0.0, 1.0, 0.0));

        self.win.swap_buffer();

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