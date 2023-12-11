trait UIComponent {
    fn render(&self) {
        println!("rendering component!");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {
    fn render(&self) {
        
    }
}

struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button {
        text: "Button a".to_owned(),
    };
    let button_b = Box::new(Button {
        text: "Button b".to_owned()
    });

    let button_c = button_a; // entire stack will be copied
    let button_d = button_b; // only box smart pointer will be copied

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d,
    ];
}
