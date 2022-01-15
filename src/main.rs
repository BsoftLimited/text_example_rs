mod grafx;

use crate::grafx::text::Collidable;
use crate::grafx::text::Text;
use crate::grafx::physics::Viewport;
use crate::grafx::Disposable;
use grafx::{ WindowHandler, WindowDetails};

struct TextObject{ text: Box<Text>,  delta_x:f32, delta_y:f32, rotation:f32 }
impl TextObject{
    pub fn new(text:&str, size:u32, x:f32, y:f32)->Self{
        let mut init = Box::new(Text::new(text));
        init.get_transform().setPosition(x, y);
        init.set_font_size(size);
        TextObject{ text:init, delta_x:120.0, delta_y:100.0, rotation:45.0 }
    }

    fn change(&mut self, port:&Viewport){
        let position_x = self.text.get_transform().get_position().getX();
        let position_y = self.text.get_transform().get_position().getY();
        self.text.set_color(
            position_x / port.get_width(),
            f32::sin(f32::to_radians(position_x * position_y)),
            position_y / port.get_height() , 1.0);
        self.rotation *= -1.0;
    }

    pub fn update(&mut self, delta:f32, port:&Viewport){
        self.text.get_transform().translate(self.delta_x * delta, self.delta_y * delta);
        self.text.get_transform().rotate(delta * self.rotation);

        let parts = self.text.get_boundary();
        for corner in parts.as_array(){
            //println!("position: x:{}, y:{}", corner.getX(), corner.getY());
            if corner.getX() <= 0.0{
                self.delta_x = 120.0;
                self.change(port);
            }else if corner.getX() >= port.get_width(){
                self.delta_x = -120.0;
                self.change(port);
            }
    
            if corner.getY() <= 0.0 {
                self.delta_y = 100.0;
                self.change(port);
            }else if corner.getY() >= port.get_height(){
                self.delta_y = -100.0;
                self.change(port);
            }
        }
    }

    pub fn draw(&self, port:&Viewport){
        self.text.draw(port);
    }
}

impl Disposable for TextObject{
    fn dispose(&mut self) {
        self.text.dispose();
    }
}

#[allow(non_snake_case)]
struct Test{ port:Box<Viewport>, texts:Vec<Box<TextObject>>, copy:Box<Text>, fps:Box<Text> }

impl Test{
    unsafe fn new()->Self{
        let mut texts = Vec::new();

        texts.push(Box::new(TextObject::new("Nobel Okelekele", 24, 400.0, 240.0)));
        texts.push(Box::new(TextObject::new("Samantha", 18, 100.0, 140.0)));
        texts.push(Box::new(TextObject::new("Rustup", 32, 10.0, 80.0)));
        texts.push(Box::new(TextObject::new("Bman", 26, 300.0, 2000.0)));
        texts.push(Box::new(TextObject::new("Edward Elric", 18, 600.0, 20.0)));
        texts.push(Box::new(TextObject::new("I Love You", 30, 200.0, 200.0)));
        texts.push(Box::new(TextObject::new("My Blessing", 22, 50.0, 50.0)));
        texts.push(Box::new(TextObject::new("Lionel Messi", 28, 100.0, 240.0)));
        texts.push(Box::new(TextObject::new("Ifeafa", 26, 400.0, 120.0)));
        texts.push(Box::new(TextObject::new("Victoria", 15, 100.0, 120.0)));
        texts.push(Box::new(TextObject::new("Ruthless", 36, 300.0, 240.0)));
        texts.push(Box::new(TextObject::new("Desiye", 20, 300.0, 240.0)));
        texts.push(Box::new(TextObject::new("School", 20, 480.0, 240.0)));
        texts.push(Box::new(TextObject::new("JRPG", 18, 400.0, 240.0)));
        texts.push(Box::new(TextObject::new("Adventure", 16, 180.0, 240.0)));
        texts.push(Box::new(TextObject::new("Java", 16, 100.0, 270.0)));
        texts.push(Box::new(TextObject::new("It Hurts", 15, 108.0, 240.0)));
        texts.push(Box::new(TextObject::new("Victoria", 15 ,50.0, 200.0)));
        texts.push(Box::new(TextObject::new("My Anchor", 15, 100.0, 240.0)));
        texts.push(Box::new(TextObject::new("Phyton", 15, 100.0, 240.0)));
        texts.push(Box::new(TextObject::new("C++", 15, 100.0, 240.0)));
        texts.push(Box::new(TextObject::new("Programming", 15, 100.0, 240.0)));

        let mut copy = Text::new("Bsoft Limited");
        copy.set_color(0.2, 0.2, 0.2, 1.0);
        let width = copy.get_width();
        let height = copy.get_height();
        copy.get_transform().setPosition(width / 2.0 + 5.0 , height / 2.0 + 5.0);

        let mut fps = Text::new("FTP: 00");
        fps.set_font_size(16);
        let fps_width = fps.get_width();
        let fps_height = fps.get_height();
        fps.get_transform().setPosition( 800.0 - fps_width - 10.0, 480.0 - fps_height - 10.0);
        fps.set_color(0.0, 0.0, 0.8, 1.0);
        Test{ port:Box::new(Viewport::new(800.0, 480.0)), texts, copy:Box::new(copy), fps:Box::new(fps) }
    }

    
}

impl WindowHandler for Test {
    fn update(&mut self, delta: f32){
        for text in &mut self.texts{
            text.update(delta, self.port.as_ref());
        }
        self.fps.set_text(format!("FPS: {}", (1.0 / delta) as i32).as_ref());
    }

    unsafe fn render(&self) {
        for text in &self.texts{
            text.draw(self.port.as_ref());
        }
        self.copy.draw(self.port.as_ref());
        self.fps.draw(self.port.as_ref());
    }

    fn resize(&mut self, width: i32, height: i32){
        //self.port.update(width as f32, height as f32);
        println!("new size width:{w}, height:{h}", w = width, h = height);
    }
}

impl Disposable for Test {
    fn dispose(&mut self) {
        for text in &mut self.texts{
            text.dispose();
        }
        self.copy.dispose();
        self.fps.dispose();
    }
}

pub fn main(){
    let details = WindowDetails::new("Text Test", 800, 480);
    let context = grafx::init(&details);
    unsafe{  grafx::start(context, Box::new(Test::new())); }
   
}