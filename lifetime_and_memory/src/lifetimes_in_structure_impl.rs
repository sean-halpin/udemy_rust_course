// https://doc.rust-lang.org/nomicon/lifetimes.html

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name)
    }
}

pub fn lifetimes_in_structure_impl() {
    let p = Person { name: "Jimbo" };
    p.talk();
}
