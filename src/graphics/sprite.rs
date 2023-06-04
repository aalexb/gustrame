use std::{mem::size_of, rc::Rc};

use gl::types::GLuint;
use glam::{Vec2, Vec3, Mat4, vec3};

use super::{texture::Texture2D, shader::ShaderProgram, buffer::{Buffer, BufferType, buffer_data}};



pub struct SpriteRenderer{
    shader:Rc<ShaderProgram>,
    quad_vao: GLuint
}

impl SpriteRenderer{
    pub fn new(shader:Rc<ShaderProgram>)-> Self{
        Self{shader,quad_vao:0}        
    }
    pub fn init_render_data(&mut self) {

        let vertices:[f32;24]=[
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        unsafe{
            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::BindVertexArray(self.quad_vao);
            let mut vbo=0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len()*size_of::<f32>()).try_into().unwrap(), vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }        
    }

    pub fn draw_sprite(&self,texture:&Texture2D, position:Vec2,size:Vec2,rotate:f32,color:Vec3) {
        unsafe{
        self.shader.apply();
        }
        let model = Mat4::IDENTITY;
        let model = model*glam::Mat4::from_translation(vec3(position.x, position.y, 0.0));

        let model = model*glam::Mat4::from_translation(vec3(0.5*size.x, 0.5*size.y, 0.0));
        let model = model*glam::Mat4::from_rotation_z(rotate.to_radians());
        let model = model*glam::Mat4::from_translation(vec3(-0.5*size.x, -0.5*size.y, 0.0));

        let model = model*glam::Mat4::from_scale(glam::vec3(size.x, size.y, 1.0));
        
        unsafe{
        self.shader.set_matrix4("model", &model);
        self.shader.set_vector3f("spriteColor", color);
        }
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0);
        }
        
        texture.bind();

        unsafe{
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

    }
}