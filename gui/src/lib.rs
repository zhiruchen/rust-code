pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for cmpt in self.components.iter() {
            cmpt.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: width: {}, height: {}, label: {}", self.width, self.height, self.label);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>, 
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing SelectBox: width: {}, height: {}, options: {:?}", self.width, self.height, self.options)
    }
}

