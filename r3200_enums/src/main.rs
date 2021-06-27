enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: f64, y: f64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my textt".to_owned());
    let click = WebEvent::Click {
        x: 20.1554,
        y: 80.332,
    };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

#[allow(dead_code)]
enum VeryLongNameOfEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

//creating alias
#[allow(dead_code)]
type Operations = VeryLongNameOfEnumOfThingsToDoWithNumbers;

impl VeryLongNameOfEnumOfThingsToDoWithNumbers {
    #[allow(dead_code)]
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y, // self is alias
            Self::Substract => x - y,
        }
    }
}
