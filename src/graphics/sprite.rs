use std::mem::size_of;

use gl::types::GLuint;
use glam::{Vec2, Vec3};

use super::{texture::Texture2D, shader::ShaderProgram, buffer::{Buffer, BufferType, buffer_data}};



pub struct SpriteRenderer{
    shader:ShaderProgram,
    quadVAO: GLuint
}

impl SpriteRenderer{
    pub fn new(shader:ShaderProgram)->Self {
        Self {shader,quadVAO:0  }
    }
    fn initRenderData(&mut self) {

        let vertices:[f32;24]=[
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];
        let vbo=Buffer::new().unwrap();
        vbo.bind(BufferType::Array);
        buffer_data(BufferType::Array, bytemuck::cast_slice(&vertices), gl::STATIC_DRAW);
        unsafe{
            gl::BindVertexArray(self.quadVAO);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, (4*size_of::<f32>()).try_into().unwrap(), 0 as *const _);
            gl::BindVertexArray(0);
        }
        vbo._unbind(BufferType::Array);
    }

    fn DrawSprite(&self,texture:&Texture2D, position:Vec2,size:Vec2,rotate:f32,color:Vec3) {
        unsafe{
        self.shader.apply();
        }
        let model = glam::Mat4::from_translation(Vec3 { x: 0.5, y: 0.5, z: 0.0});
        let model =model*glam::Mat4::from_scale(glam::vec3(size.x, size.y, 1.0));





        unsafe{
        self.shader.SetMatrix4("model", &model);
        self.shader.SetVector3f("spriteColor", color);
        }
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0);
        }
        
        texture.bind();

        unsafe{
            gl::BindVertexArray(self.quadVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

    }
}