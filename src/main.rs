fn main() {
    let sam = Person::new(String::from("Samwel Charles"), 33);

    println!("{sam:?}");

    create_default()
}

fn create_default() {
    let tmp = Person {
        age: 33,
        ..Default::default()
    };

    println!("{tmp:?}")
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "user".to_string(),
            age: 0,
        }
    }
}
