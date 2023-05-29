use gl::types::*;
pub struct Texture2D{
    id:GLuint,
    width:u32,
    height:u32,
    internal_format:GLenum,
    image_format:GLenum,
    wrap_s:GLenum,
    wrap_t:u32,
    filter_min:u32,
    filter_max:u32,
}

impl Texture2D {
    pub fn new()->Self {
        let mut one=Self { 
            id: 0, 
            width: 0, 
            height: 0, 
            internal_format: gl::RGB, 
            image_format: gl::RGB, 
            wrap_s: gl::REPEAT, 
            wrap_t: gl::REPEAT, 
            filter_min: gl::LINEAR, 
            filter_max: gl::LINEAR };
        unsafe{
            gl::GenTextures(1, &mut one.id);
        }
        return one;
    }
    pub fn generate(&mut self, width:u32, height:u32, data: &[u8]) {
        self.width=width;
        self.height=height;
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D, 
                0, 
                self.internal_format as i32, 
                width as i32, 
                height as i32, 
                0, 
                self.image_format, 
                gl::UNSIGNED_BYTE, 
                data.as_ptr().cast());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_max as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    pub fn bind(&self){
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}