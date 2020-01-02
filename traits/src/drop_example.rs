#[allow(unused_assignments)]
#[allow(unused_variables)]

// equivalent to destructor

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

pub fn drop_example() {
    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        clever = goblin;
        println!("end of scope");
        // drop(goblin);
    }
}
