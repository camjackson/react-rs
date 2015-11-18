extern crate react;

use react::Component;

fn main() {
    let div = react::Div { class_name: "container", children: Vec::new() };

    println!("{}", div.render());
}
