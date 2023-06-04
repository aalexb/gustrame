use std::{fs::File, io::{BufReader, BufRead}};

use glam::{vec2, Vec2, vec3, Vec3};

use crate::graphics::{interface::ITextureCache, sprite::SpriteRenderer};

use super::gameobject::GameObject;


pub struct GameLevel{
    pub bricks:Vec<GameObject>,
    is_completed:bool,
}

impl GameLevel {
    pub fn new()->Self {
        Self { bricks: Vec::new(), is_completed: false }
    }
    pub fn load(&mut self,file:&str,level_width:u32,level_height:u32,res:&impl ITextureCache) {
        self.bricks.clear();
        let tile_code:u32;
        let level:Self;
        let line=String::new();
        let file = File::open(file).unwrap();
        let stream = BufReader::new(file).lines();
        let mut tile_data:Vec<Vec<u32>> = Vec::new();
        for line in stream {
            let mut row:Vec<u32>=Vec::new();
            for ch in line.unwrap().chars() {
                if let Some(a) =char::to_digit(ch, 10)  {
                    row.push(a)
                }                
            }
            tile_data.push(row);
        }
        if tile_data.len()>0 {
            self.init(tile_data, level_width, level_height,res)
        }
    }
    pub fn draw(&self, renderer:&SpriteRenderer) {
        for tile in &self.bricks {
            if !tile.destroyed() {
                tile.draw(renderer)
            }
        }
    }
    fn is_completed(&self)->bool {
        for tile in &self.bricks {
            if !tile.is_solid()&&!tile.destroyed() {
                return false
            }
        }
        return true;
    }


    fn init(&mut self, tile_data:Vec<Vec<u32>>,level_width:u32,level_height:u32,res:&impl ITextureCache) {
        let height = tile_data.len();
        let width = tile_data[0].len();
        let unit_width = level_width as f32/width as f32;
        let unit_height = level_height as f32/ height as f32;
        for y in 0..height {
            for x in 0..width {
                match tile_data[y][x] {
                    0=>{}
                    1=>{
                        let pos =vec2(unit_width*x as f32,unit_height*y as f32);
                        let size = vec2(unit_width,unit_height);
                        let mut obj=GameObject::new(pos, size, Vec2::ZERO, vec3(0.8,0.8,0.7), 
                        res.get_texture("block_solid"));
                        obj.set_solid(true);
                        self.bricks.push(obj);
                    }
                    abc=>{
                        let mut color=Vec3::ONE;
                        match abc {
                            2=>color=vec3(0.2,0.6,1.0),
                            3=>color=vec3(0.0,0.7,0.0),
                            4=>color=vec3(0.8,0.8,0.4),
                            5=>color=vec3(1.0,0.5,0.0),
                            _=>{}
                        }
                        let pos =vec2(unit_width*x as f32,unit_height*y as f32);
                        let size = vec2(unit_width,unit_height);
                        let obj=GameObject::new(pos, size, Vec2::ZERO, color, 
                        res.get_texture("block"));
                        self.bricks.push(obj);

                    }
                }
            }
        }
    }
}
