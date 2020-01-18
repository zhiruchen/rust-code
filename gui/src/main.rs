extern crate gui;

use gui::{Screen, SelectBox, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Yes"),String::from("Maybe"),String::from("No")],
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("button"),
            }),
        ],
    };

    // dynamic dispatch,
    // at runtime, Rust uses the pointers inside the trait object to know which method to call.
    screen.run();
}
