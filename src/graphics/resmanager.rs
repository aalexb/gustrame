use std::{collections::HashMap, str::FromStr, fs::File, io::Read, rc::Rc};

use image::GenericImageView;

use super::{shader::{ShaderProgram, Shader}, texture::Texture2D};

pub struct ResManager{
    shaders:HashMap<String,Rc<ShaderProgram>>,
    textures:HashMap<String,Texture2D>,    
}

impl ResManager {
    pub fn new() -> Self { 
            Self { shaders:HashMap::new(), textures:HashMap::new() } 
        }
    pub fn LoadShader(&mut self, vshaderfile:&str,fshaderfile:&str,name:&str) {
        let mut vshader=String::new();
        let mut fshader=String::new();
        ResManager::loadshaderfromfile(vshaderfile, &mut vshader);
        ResManager::loadshaderfromfile(fshaderfile, &mut fshader);
        unsafe{
            let shs=[
                Shader::new(vshader.as_str(), gl::VERTEX_SHADER).unwrap(),
                Shader::new(fshader.as_str(), gl::FRAGMENT_SHADER).unwrap()];  
            self.shaders.insert(String::from_str(name).unwrap(),Rc::new( ShaderProgram::new(&shs).unwrap()));          
        }       
    }
    pub fn GetShader(&self, name:&str)->Rc<ShaderProgram> {
        self.shaders.get(name).unwrap().clone()
        
    }
    pub fn LoadTexture(&mut self,file:&str,alpha:bool, name:&str) {
        self.textures.insert(String::from_str(name).unwrap(), self.loadtexturefromfile(file,alpha));
    }

    fn loadtexturefromfile(&self,file:&str, alpha:bool)->Texture2D {
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

    pub fn GetTexture(&self, name:&str)->&Texture2D {
        self.textures.get(name).unwrap().to_owned()
    }

    fn loadshaderfromfile(ShaderFile:&str,buf:&mut String) {
        let mut file = File::open(ShaderFile).unwrap();
        let mut s = String::new();
        let x=file.read_to_string(buf).unwrap();
    }

}

