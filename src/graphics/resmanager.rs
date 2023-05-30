use std::{collections::HashMap, str::FromStr};

use image::GenericImageView;

use super::{shader::{ShaderProgram, Shader}, texture::Texture2D};

pub struct ResManager{
    shaders:HashMap<String,ShaderProgram>,
    textures:HashMap<String,Texture2D>,    
}

impl ResManager {
    pub fn new(shaders: HashMap<String,ShaderProgram>, 
        textures: HashMap<String,Texture2D>) -> Self { 
            Self { shaders:HashMap::new(), textures:HashMap::new() } 
        }
    pub fn LoadShader(&self, vshader:&str,fshader:&str,name:&str) {
        unsafe{
            let shs=[
                Shader::new(vshader, gl::VERTEX_SHADER).unwrap(),
                Shader::new(fshader, gl::FRAGMENT_SHADER).unwrap()];  
            self.shaders.insert(String::from_str(name).unwrap(), ShaderProgram::new(&shs).unwrap());          
        }       
    }
    pub fn GetShader(&self, name:&str)->ShaderProgram {
        *self.shaders.get(name).unwrap().to_owned()
    }
    pub fn LoadTexture(&self,file:&str,alpha:bool, name:&str) {
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

    pub fn GetTexture(&self, name:&str)->Texture2D {
        *self.textures.get(name).unwrap().to_owned()
    }
}

