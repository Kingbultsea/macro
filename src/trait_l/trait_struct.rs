trait Draw {
    fn draw(&self);
}

struct SelectBox {
    height: i32,
    width: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        for item in &self.options {
            println!("SelectBox: {}", item);
        }
    }
}

struct Button {
    height: i32,
    width: i32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        print!("Buttton: {}", self.label);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

pub fn run_trait_struct() {
    let screens = Screen {
        components: vec![
            Box::new(Button {
                height: 1,
                width: 2,
                label: String::from("123"),
            }),
            Box::new(SelectBox {
                width: 2,
                height: 2,
                options: vec![String::from("456")],
            }),
        ],
    };
}
