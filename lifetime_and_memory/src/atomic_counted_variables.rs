use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn atomic_counted_variables() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);
    t.join().unwrap();
}
