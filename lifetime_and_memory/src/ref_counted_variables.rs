use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn ref_counted_variables() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, Ref Count = {}", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, Ref Count = {}", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, Ref Count = {}", name, Rc::strong_count(&name));
}
