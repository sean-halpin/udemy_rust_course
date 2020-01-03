// https://doc.rust-lang.org/nomicon/lifetimes.html

struct Person {
    name: String,
}

impl Person {
    // fn get_ref_name<'a>(&'a self) -> &'a String {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

// 'z defines a lifetime & marking ceo as the same lifetime
// tells the compiler that ceo will live as long as the company
struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

pub fn lifetimes() {
    let boss = Person {
        name: String::from("Elon"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    let mut z: &String;
    {
        let p = Person {
            name: String::from("Jimbo"),
        };
        z = p.get_ref_name();
    }
}
