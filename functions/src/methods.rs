struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn methods() {
    let a = Point { x: 1.2, y: 3.4 };
    let b = Point { x: 5.2, y: 9.4 };
    let l = Line { start: a, end: b };

    println!("length = {}", l.len());
}
