struct Point<T> {
    x: T,
    y: T,
}

struct PointTV<T, V> {
    x: T,
    y: V,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
} 

pub fn generics() {
    let a = Point { x: 1.2, y: 3.4 };
    let b = Point { x: 8.9, y: 5.4 };
    let c: Point<f64> = Point { x: 1.2, y: 3.4 };
    let d: PointTV<u16, u32> = PointTV { x: 0, y: 0 };

    let my_line = Line { start: a, end: b};
    println!("{:?}", my_line.start.x);
}
