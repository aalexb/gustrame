use std::{rc::Rc, fs::File, io::{BufReader, BufRead}};

use glam::{Vec2, vec3, Vec3, vec2};
mod gamelevel;
mod gameobject;
mod utils;

use crate::{graphics::{*, sprite::SpriteRenderer, texture::Texture2D, interface::ITextureCache}, resmanager::ResManager};

use self::{gamelevel::GameLevel, gameobject::{GameObject, Movable, Ball}, utils::{AABB, check_collision}};

const PLAYER_SIZE:Vec2=vec2(100.0,20.0);
const PLAYER_VELOCITY:f32 = 1000.0;
const INIT_BALL_VELOCITY:Vec2 = vec2(100.0,-350.0);
const BALL_RADIUS:f32 = 12.5;

pub struct Game{
    state:GameState,
    pub keys:[bool;1024],
    width:u32,
    height:u32,
    win:Box<dyn interface::IWindow>,
    last_frame:f64,
    resmgr:ResManager,
    renderer:SpriteRenderer,
    levels:Vec<GameLevel>,
    level:u32,
    player:GameObject,
    ball:Ball,
}

impl Game{
    pub fn new(width:u32, height:u32)->Self {
        let window = create_window(width,height);
        gl_load(width as i32,height as i32);

        let mut resmgr = ResManager::new();

        resmgr.load_shader("shader.vert", "shader.frag", "sprite");
        resmgr.load_shader("simpleshader.vert", "simpleshader.frag", "simple");
        
        
        
        let mut renderer = SpriteRenderer::new(resmgr.get_shader("sprite"));
        renderer.init_render_data();

        Self { state: GameState::GameActive, 
            keys: [false;1024], width, height, 
            win:Box::new(window),last_frame:0.0,resmgr,renderer,levels:Vec::new(),level: 0,
        player:GameObject::default(),ball:Ball::default()}
    }

    pub fn init(&mut self) {
        self.resmgr.load_texture("background.jpg", false, "background");
        self.resmgr.load_texture("awesomeface.png", true, "ball");
        self.resmgr.load_texture("block.png", true, "block");
        self.resmgr.load_texture("block_solid.png", true, "block_solid");
        self.resmgr.load_texture("paddle.png", true, "paddle");

        let mut one = GameLevel::new();
        one.load("one.lvl", self.width, self.height/2, &self.resmgr);
        let mut two = GameLevel::new();
        two.load("two.lvl", self.width, self.height/2, &self.resmgr);
        let mut three = GameLevel::new();
        three.load("three.lvl", self.width, self.height/2, &self.resmgr);
        let mut four = GameLevel::new();
        two.load("two.lvl", self.width, self.height/2, &self.resmgr);

        self.levels.push(one);
        self.levels.push(two);
        self.levels.push(three);
        self.levels.push(four);


        let player_pos=vec2(
            self.width as f32/2.0-PLAYER_SIZE.x/2.0,
            self.height as f32-PLAYER_SIZE.y);
        self.player=GameObject::new(player_pos, PLAYER_SIZE, vec2(PLAYER_VELOCITY,0.0), Vec3::ONE, 
            self.resmgr.get_texture("paddle"));

        let ball_pos=player_pos+vec2(PLAYER_SIZE.x/2.0-BALL_RADIUS,
            -BALL_RADIUS*2.0);
        self.ball=Ball::new(ball_pos, BALL_RADIUS, INIT_BALL_VELOCITY, self.resmgr.get_texture("ball"));

    }

    pub fn run(&mut self) {
        while self.state==GameState::GameActive {
            let current_frame=self.win.get_time();
            let delta_time=current_frame-self.last_frame;
            self.last_frame=current_frame;
            self.process_input(delta_time);
            self.update(delta_time);
            self.render();
            println!("dt={},cur_frame={},lastframe={}",delta_time,current_frame,self.last_frame);
        }
    }
    
    fn process_input(&mut self,dt:f64) {
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
        if self.state==GameState::GameActive {
            let velocity = PLAYER_VELOCITY*dt as f32;
            if self.keys[65] {
                if self.player.position().x>=0.0 {
                    self.player.set_position_x(self.player.position().x-velocity);
                    if self.ball.stuck {
                        self.ball.obj.set_position_x(self.player.position().x-velocity)
                    }
                }
            }
            if self.keys[68] {
                if self.player.position().x<=self.width as f32-self.player.size().x {
                    self.player.set_position_x(self.player.position().x+velocity);
                    if self.ball.stuck {
                        self.ball.obj.set_position_x(self.player.position().x+velocity)
                    }
                }
            }
            if self.keys[32] {
                self.ball.stuck=false;
            }
        }
    }
    
    fn update(&mut self,dt:f64) {
        self.ball.do_move(dt as f32, self.width);
        self.do_collisions();
    }
    
    fn render(&mut self) {

        gl_render();
        let projection = glam::Mat4::orthographic_rh_gl(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        unsafe{
            self.resmgr.get_shader("sprite").apply().set_integer("image", 0);
            self.resmgr.get_shader("sprite").set_matrix4("projection", &projection);            
        }
        if self.state==GameState::GameActive {
            self.renderer.draw_sprite(self.resmgr.get_texture("background").as_ref(), vec2(0.0,0.0), vec2(self.width as f32,self.height as f32), 0.0, Vec3::ONE);
            self.levels[self.level as usize].draw(&self.renderer);
        }
        self.player.draw(&self.renderer);
        self.ball.obj.draw(&self.renderer);
        self.win.swap_buffer();

    }

    fn state(&self)->GameState {
        self.state
    }

    fn do_collisions(&mut self) {
        for boxy in &mut self.levels[self.level as usize].bricks {
            if !boxy.destroyed() {
                if check_collision(&self.ball, boxy) {
                    if !boxy.is_solid() {
                        boxy.set_destroyed(true)
                    }
                }
            }
        }
    }
}
#[derive(Clone,Copy,PartialEq)]
pub enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}

pub enum Direction{
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

