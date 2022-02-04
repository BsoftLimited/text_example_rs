pub mod shader;

use crate::grafx::Disposable;
use crate::grafx::physics::Color;
use crate::grafx::materials::shader::Shader;

pub struct MaterialProperty{ diffuse:Box<Color>, ambient:Box<Color>, specular:Box<Color>, shinines:f32 }

#[allow(dead_code)]
impl MaterialProperty{
    pub fn new()->Self{
        MaterialProperty{
            diffuse:Box::new(Color::white()), ambient:Box::new(Color::white()), specular:Box::new(Color::white()), shinines:127.0
        }
    }
    pub fn get_diffuse_color(&self)->&Box<Color>{ &self.diffuse }
    pub fn get_ambient_color(&self)->&Box<Color>{ &self.ambient }
    pub fn get_specular_color(&self)->&Box<Color>{ &self.specular }
    pub fn get_shininess(&self)->f32{ self.shinines}
}

pub trait Material : Disposable{
    fn get_shader(&self)->&Box<Shader>;
    fn get_properties(&self)->&Box<MaterialProperty>;
    unsafe fn r#use(&self){
        let init = self.get_properties();
        self.get_shader().bind();
        self.get_shader().set_uniform_color("material.specular", &init.get_specular_color());
		self.get_shader().set_uniform_color("material.diffuse", &init.get_diffuse_color());
		self.get_shader().set_uniform_color("material.ambient", &init.get_ambient_color());
		self.get_shader().set_uniform_value("material.shininess", init.get_shininess());
    }

    fn set_diffuse_color(&mut self, color:Color);
    fn set_ambient_color(&mut self, color:Color);
    fn set_specular_color(&mut self, color:Color);
    fn set_shininess(&mut self, shinines:f32);
}

pub struct BasicMaterial{ shader:Box<Shader>, properties:Box<MaterialProperty>}

#[allow(dead_code)]
impl BasicMaterial{
    pub unsafe fn new()->Self{
        let shader = Shader::simple();
        BasicMaterial{ shader:Box::new(shader), properties:Box::new(MaterialProperty::new()) }
    }
}

impl Material for BasicMaterial {
    fn get_shader(&self) -> &Box<Shader> { &self.shader }
    fn get_properties(&self) -> &Box<MaterialProperty> { &self.properties }

    fn set_diffuse_color(&mut self, color:Color){ self.properties.diffuse = Box::new(color); }
    fn set_ambient_color(&mut self, color:Color){ self.properties.ambient = Box::new(color); }
    fn set_specular_color(&mut self, color:Color){ self.properties.specular = Box::new(color); }
    fn set_shininess(&mut self, shinines:f32){ self.properties.shinines = shinines; }
}

impl Disposable for BasicMaterial{
    fn dispose(&mut self) {
        unsafe { self.get_shader().dispose(); }
    }
}