use std::{mem::size_of, f32::consts::PI};

use gl::types::{GLuint};
use glam::Vec3;

pub struct Mesh{
    vao:GLuint,
    vertices:Vec<Vertex>,
}

impl Mesh{
    fn new()->Self {
        Self { vao: 0, vertices: Vec::new() }
    }

    fn gen_vao(&mut self) {
        unsafe{
            gl::GenVertexArrays(1,&mut self.vao);
            gl::BindVertexArray(self.vao);
            let mut vbo=0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (self.vertices.len()*size_of::<Vertex>()).try_into().unwrap(), 
                self.vertices.as_ptr().cast(), 
                gl::STATIC_DRAW
            );            
            gl::VertexAttribPointer(
                0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                size_of::<Vertex>().try_into().unwrap(), 
                0 as *const _);

            gl::VertexAttribPointer(
                1, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                size_of::<Vertex>().try_into().unwrap(), 
                size_of::<Vec3>() as *const _);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::BindVertexArray(0);
        }
    }


    pub fn circle(radius: f32)->Self {
        let mut mesh = Self::new();
        let approx = 60;
        let pos = Vec3::ZERO;
        let color = Vec3::new(((0_f32).sin()+1.0)/2.0,((0_f32).sin()+1.0)/2.0,0.0);
        mesh.vertices.push(Vertex { pos, color});
        for i in 0..=approx {
            
            let angle = (i as f32)*2.0*PI/(approx as f32);
            let pos = Vec3::new(radius*angle.cos(),radius*angle.sin(), 0.0);
            let color_bit = (angle.sin()+1.0)/2.0;
            let color = Vec3::new(color_bit,color_bit,0.0);
            mesh.vertices.push(Vertex { pos, color});
        }        
        mesh.gen_vao();        
        
        return mesh;
    }
    pub fn draw(&self) {
        unsafe{
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }
}

pub enum DrawType {
    TriStrip=gl::TRIANGLE_STRIP as isize,
    Tri=gl::TRIANGLES as isize,
    TriFan=gl::TRIANGLE_FAN as isize,

}

pub struct Vertex{
    pos:Vec3,
    color:Vec3
}