use freetype::freetype_sys::FT_Vector;

pub struct Bearing(pub i32, pub i32);

pub struct Size{ width:i32, height:i32}
impl Size{
    pub fn new(width:i32, height:i32)->Self{
        Size{ width, height}
    }

    pub fn get_width(&self)->i32{ self.width }
    pub fn get_height(&self)->i32{ self.height }
}

pub struct Character{
    texture_id : u32,
    size: Size,      // Size of glyph
    bearing: Bearing,   // Offset from baseline to left/top of glyph
    advance: FT_Vector 
}

impl Character{
    pub fn new(texture_id:u32, width:i32, height:i32, x:i32, y:i32, advance:FT_Vector)->Self{
        Character{ texture_id, size:Size::new(width, height), bearing:Bearing(x, y), advance }
    }

    pub fn get_texture(&self)->u32{ self.texture_id }
    pub fn get_size(&self)->&Size{ &self.size }
    pub fn get_bearing(&self)->&Bearing{ &self.bearing }
    pub fn get_advance(&self)->&FT_Vector{ &self.advance }
}
