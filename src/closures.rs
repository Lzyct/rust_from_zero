use std::thread;
use std::time::Duration;

pub fn closures() {
    let intensity = 20;
    let random_number = 5;
    generate_workout(intensity, random_number);

    let x = 5;
    // create function as variable
    let equal_to_x = |z: i32| z == x;

    // different with function, closure can capture environment
    // fn equal_to_x2(z: i32) -> bool {
    //     z == x // error can't capture dynamic environment in a fn item
    // }

    println!("equal_to_x: {}", equal_to_x(5));
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}