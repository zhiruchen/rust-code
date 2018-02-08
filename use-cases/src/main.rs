pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested modules!");
            }
        }
    }
}

#[derive(Debug)]
enum TraficLight {
    Red,
    Yellow,
    Green,
}

use TraficLight::{Red, Yellow};
use a::series::of; // use 引入想要调用的函数所在的模块

fn main() {
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TraficLight::Green;
    println!("{:?}, {:?}, {:?}", red, yellow, green);
}