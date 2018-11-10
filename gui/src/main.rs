extern crate gui;
use gui::Draw;
use gui::Button;
use gui::Screen;


#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height:u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box...
    }
}


fn main () {
    let sb = SelectBox {width: 10, height: 100, options: vec!["yes".into(), "no".into()]};
    let button = Button {width: 35, height: 60, label: "click me".into()};
    sb.draw();
    println!("select box: {:?}", sb);

    let screen = Screen {
        components: vec![Box::new(sb), Box::new(button)],
    };

    screen.run();
    
}