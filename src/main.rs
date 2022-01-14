mod grafx;

use crate::grafx::text::Text;
use crate::grafx::physics::Viewport;
use crate::grafx::Disposable;
use grafx::{ WindowHandler, WindowDetails};

#[allow(non_snake_case)]
struct Test{
    port:Box<Viewport>, name:Box<Text>, copy:Box<Text>, delta_x:f32, delta_y:f32
}

impl Test{
    unsafe fn new()->Self{
        let mut init = Text::new("Nobel Okelekele");
        init.set_font_size(36);
        init.get_transform().setPosition(400.0, 240.0);

        let mut copy = Text::new("Bsoft Limited");
        copy.set_color(0.2, 0.2, 0.2, 1.0);
        let width = copy.get_width();
        let height = copy.get_height();
        copy.get_transform().setPosition(width / 2.0 , height / 2.0);
        Test{
            port:Box::new(Viewport::new(800.0, 480.0)), name:Box::new(init), copy:Box::new(copy), delta_x:-120.0, delta_y:-100.0
        }
    }

    fn change(&mut self){
        let position_x = self.name.get_transform().get_position().getX();
        let position_y = self.name.get_transform().get_position().getY();
        self.name.set_color(
            position_x / self.port.get_width(),
            f32::sin(f32::to_radians(position_x * position_y)),
            position_y / self.port.get_height() , 1.0);
    }
}

impl WindowHandler for Test {
    fn update(&mut self, delta: f32){
        self.name.get_transform().translate(self.delta_x * delta, self.delta_y * delta);
        self.name.get_transform().rotate(45.0 * delta);
        let position_x = self.name.get_transform().get_position().getX();
        let position_y = self.name.get_transform().get_position().getY();
        let width = self.name.get_width() / 2.0;
        let height = self.name.get_height() / 2.0;

        if position_x <= width || position_x + width >= self.port.get_width(){
            self.delta_x *= -1.0;
            self.change();
        }

        if position_y <= height || position_y + height >= self.port.get_height(){
            self.delta_y *= -1.0;
            self.change();
        }
    }

    unsafe fn render(&self) {
        self.name.draw(&self.port);
        self.copy.draw(&self.port);
    }

    fn resize(&mut self, width: i32, height: i32){
        //self.port.update(width as f32, height as f32);
        println!("new size width:{w}, height:{h}", w = width, h = height);
    }
}

impl Disposable for Test {
    fn dispose(&self) {}
}

pub fn main(){
    let details = WindowDetails::new("Text Test", 800, 480);
    let context = grafx::init(&details);
    unsafe{  grafx::start(context, Box::new(Test::new())); }
   
}