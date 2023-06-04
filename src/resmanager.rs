use std::{collections::HashMap, str::FromStr, fs::File, io::Read, rc::Rc};

use image::GenericImageView;

use crate::graphics::{shader::{ShaderProgram, Shader}, texture::Texture2D, interface::ITextureCache};



pub struct ResManager{
    shaders:HashMap<String,Rc<ShaderProgram>>,
    textures:HashMap<String,Rc<Texture2D>>,    
}

impl ResManager {
    pub fn new() -> Self { 
            Self { shaders:HashMap::new(), textures:HashMap::new() } 
        }
    pub fn load_shader(&mut self, vshaderfile:&str,fshaderfile:&str,name:&str) {
        let mut vshader=String::new();
        let mut fshader=String::new();
        ResManager::load_shader_from_file(vshaderfile, &mut vshader);
        ResManager::load_shader_from_file(fshaderfile, &mut fshader);
        unsafe{
            let shs=[
                Shader::new(vshader.as_str(), gl::VERTEX_SHADER).unwrap(),
                Shader::new(fshader.as_str(), gl::FRAGMENT_SHADER).unwrap()];  
            self.shaders.insert(String::from_str(name).unwrap(),Rc::new( ShaderProgram::new(&shs).unwrap()));          
        }       
    }
    pub fn get_shader(&self, name:&str)->Rc<ShaderProgram> {
        self.shaders.get(name).unwrap().clone()
        
    }
    pub fn load_texture(&mut self,file:&str,alpha:bool, name:&str) {
        self.textures.insert(String::from_str(name).unwrap(),Rc::new( self.load_texture_from_file(file,alpha)));
    }

    fn load_texture_from_file(&self,file:&str, alpha:bool)->Texture2D {
        let mut texture:Texture2D=Texture2D::new();
        if alpha {
            texture.internal_format=gl::RGBA;
            texture.image_format=gl::RGBA;
        }
        let img=image::open(file).unwrap();
        let (width,height) = img.dimensions();
        texture.generate(width, height, img.as_bytes());
        return texture
    }

    fn load_shader_from_file(shader_file:&str,buf:&mut String) {
        let mut file = File::open(shader_file).unwrap();
        file.read_to_string(buf).unwrap();
    }

}

impl ITextureCache for ResManager {
    fn get_texture(&self,name:&str)->Rc<Texture2D> {
        Rc::clone(self.textures.get(name).unwrap())
    }
}