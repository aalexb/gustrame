use gl::types::{GLuint, GLenum};

//#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum BufferType{
    Array = gl::ARRAY_BUFFER as isize,
    _ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new()->Option<Self> {
        let mut vbo = 0;
        unsafe{
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo!=0 {
            Some(Self(vbo))
        }else{
            None
        }
    }

    pub fn bind(&self, ty:BufferType) {
        unsafe {gl::BindBuffer(ty as GLenum, self.0)}
    }

    pub fn _unbind(&self, ty:BufferType) {
        unsafe {gl::BindBuffer(ty as GLenum, 0)}
    }
}

pub fn buffer_data(ty: BufferType, data: &[f32], usage: GLenum) {
    unsafe {
        gl::BufferData(
            ty as GLenum, 
            data.len().try_into().unwrap(), 
            data.as_ptr().cast(), 
            usage
        );
    }    
}