pub fn iterators() {
    let v1 = vec![1, 2, 3, 4, 5];
    let i1 = v1.iter();

    let total: i32 = i1.sum();
    println!("total: {}", total);

    // for number in i1 {
    //     println!("num {}", number);
    // }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v1_iterator = v1.iter();

    assert_eq!(v1_iterator.next(), Some(&1));
    assert_eq!(v1_iterator.next(), Some(&2));
    assert_eq!(v1_iterator.next(), Some(&3));
    assert_eq!(v1_iterator.next(), Some(&4));
    assert_eq!(v1_iterator.next(), Some(&5));
    assert_eq!(v1_iterator.next(), None);
}

#[test]
fn iterator_map_closures() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4, 5, 6]);
}

fn filter_shoes_size(shoes: Vec<Shoes>, size: i32) -> Vec<Shoes> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[derive(PartialEq, Debug)]
struct Shoes {
    size: i32,
    style: String,
}

#[test]
fn test_shoes_size() {
    let shoes = vec![
        Shoes { size: 10, style: String::from("sneaker") },
        Shoes { size: 13, style: String::from("sandal") },
        Shoes { size: 10, style: String::from("boot") },
    ];

    let in_my_size = filter_shoes_size(shoes, 10);

    assert_eq!(in_my_size, vec![
        Shoes {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoes {
            size: 10,
            style: String::from("boot"),
        },
    ]);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // associated type
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_iterator_test() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn use_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(2))
        .map(|(a, b)| {
            println!("a: {}, b: {}", a, b);
            a * b
        })
        .map(|x| {
            println!("x: {}", x);
            println!("-------------------");
            x
        })
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum); // 3 + 15
}