use std::{ffi::{CString, NulError}, ptr};

use gl::types::*;
pub struct Shader{
    pub id:GLuint,
}

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type:GLenum)->Result<Self, NulError> {
        let source_code =CString::new(source_code)?;
    let shader=Self{id: gl::CreateShader(shader_type)};

    gl::ShaderSource(
        shader.id, 
        1, 
        &source_code.as_ptr(), 
        ptr::null());
        gl::CompileShader(shader.id);
        let mut success = 0;

        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        if success==1 {
            Ok(shader)
        }else {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(shader.id, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe{
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram{
    pub id:GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader])->Result<Self,NulError> {
        let program =Self{id:gl::CreateProgram()};
        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        };
        gl::LinkProgram(program.id);
        let mut success = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        if success==1 {
            Ok(program)
        }else{
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(program.id, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe{
            gl::DeleteProgram(self.id);
        }
    }
}