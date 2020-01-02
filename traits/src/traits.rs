trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
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
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow", self.name())
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self {
            result += *x
        }
        return result;
    }
}

pub fn traits() {
    let h = Human::create("John");
    h.talk();
    let c = Cat::create("Mittens");
    c.talk();
    let h2: Human = Animal::create("Jim");
    h2.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}
