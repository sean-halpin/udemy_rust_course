trait Animal {
    // fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Hello", self.name())
    }
}

struct Cat {
    name: &'static str,
}
impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

pub fn vectors_of_diff_objects() {
    let mut creatures: Vec<Creature> = Vec::new();
    // creatures.push(Human { name: "Jimbo" });
    // creatures.push(Cat { name: "Mittens" });
    creatures.push(Creature::Human(Human { name: "Jimbo" }));
    creatures.push(Creature::Cat(Cat { name: "Mittens" }));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    //
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Jimbo" }));
    animals.push(Box::new(Cat { name: "Mittens" }));
    for a in animals.iter() {
        a.talk();
    }
}
