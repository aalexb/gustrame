use std::rc::Rc;

use glam::{Vec2, Vec3, vec2};

use crate::graphics::{texture::Texture2D, sprite::SpriteRenderer};

pub struct GameObject{
    position:Vec2,
    size:Vec2,
    velocity:Vec2,
    color:Vec3,
    rotation:f32,
    is_solid:bool,
    destroyed:bool,
    sprite: Rc<Texture2D>
}

impl Movable for GameObject {
    fn do_move(&self,dt:f64,width:f32) {
        todo!()
    }

    fn is_stuck(&self)->bool {
        todo!()
    }
}
impl GameObject{
    pub fn default()->Self {
        Self { 
            position: Vec2::ZERO, 
            size: Vec2::ONE, 
            velocity: Vec2::ZERO, 
            color: Vec3::ONE, 
            rotation: 0.0, 
            is_solid: false, 
            destroyed: false, 
            sprite:Rc::new( Texture2D::new()) }
    }
    pub fn new(position: Vec2, size: Vec2, velocity: Vec2, 
        color: Vec3, sprite: Rc<Texture2D>) -> Self { 
        Self { 
            position, 
            size, 
            velocity, 
            color, 
            rotation:0.0, 
            is_solid:false, 
            destroyed:false, 
            sprite } 
    }


    pub fn draw(&self,renderer:&SpriteRenderer) {
        renderer.draw_sprite(&self.sprite, self.position, self.size, self.rotation, self.color)
    }

    pub fn destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn is_solid(&self) -> bool {
        self.is_solid
    }

    pub fn set_solid(&mut self, solid: bool) {
        self.is_solid = solid;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position_x(&mut self, position: f32) {
        self.position.x = position;
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn set_destroyed(&mut self, destroyed: bool) {
        self.destroyed = destroyed;
    }
}

trait DrawGameObject {
    
}

pub trait Movable {
    fn do_move(&self,dt:f64,width:f32);
    fn is_stuck(&self)->bool;
}

pub struct Ball{
    pub radius:f32,
    pub stuck:bool,
    pub obj:GameObject,
}

impl Ball {
    pub fn default()->Self {
        Self { radius: 0.0, stuck: true, obj: GameObject::default() }
    }
    pub fn new(position: Vec2, radius: f32,velocity:Vec2,texture:Rc<Texture2D>) -> Self { 
        Self { radius, stuck:true, 
            obj:GameObject::new(position,vec2(radius*2.0,radius*2.0),
            velocity,Vec3::ONE,texture) }
        }
    pub fn do_move(&mut self, dt:f32, window_width:u32)->Vec2 {
        if !self.stuck {
            self.obj.position+=self.obj.velocity*dt;
            if self.obj.position.x<=0.0 {
                self.obj.velocity.x=-self.obj.velocity.x;
                self.obj.position.x = 0.0;
            }
            else if self.obj.position.x+self.obj.size.x>=window_width as f32 {
                self.obj.velocity.x=-self.obj.velocity.x;
                self.obj.position.x = window_width as f32-self.obj.size.x;
            }
            if self.obj.position.y<=0.0 {
                self.obj.velocity.y=-self.obj.velocity.y;
                self.obj.position.y=0.0;
            }
        }
        return self.obj.position
    }
    pub fn reset(&mut self,position:Vec2,velocity:Vec2) {
        self.obj.position=position;
        self.obj.velocity=velocity;
        self.stuck=true;
    }
}