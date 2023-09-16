pub fn generic_types() {
    let list_int = vec![10, 230, 300, 451, 250];

    let largest = get_largest(list_int);

    println!("largest: {}", largest);

    let p1 = Point { x: 1, y: 2 };
    let _p2 = Point { x: 2.1, y: 5 };
    let p3 = Point { x: 'a', y: 10.5 };
    let p4 = p1.mixup(p3);
    println!("p3 x: {} ,y: {}", &p4.x, p4.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}


impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn get_largest<T: PartialOrd + Copy>(list_int: Vec<T>) -> T {
    let mut largest = list_int[0];

    for number in list_int {
        if number > largest {
            largest = number;
        }
    }
    largest
}