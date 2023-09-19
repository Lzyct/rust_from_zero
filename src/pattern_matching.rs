#[derive(Debug)]
enum Language {
    English,
    Bahasa,
    Malay,
    Russian,
    Japanese,
}

pub fn pattern_matching() {
    let language = Language::Japanese;

    match language {
        Language::English => println!("Hello"),
        Language::Bahasa => println!("Hai"),
        Language::Malay => println!("Apa khabar"),
        Language::Russian => println!("Privet"),
        unknown => println!("Unsupported language {:?}", unknown),
    }

    let v1 = vec![1, 2, 3, 4, 5];

    for (index, value) in v1.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    let lat_lng = (3, 5);
    get_coordinates(&lat_lng);
}

fn get_coordinates(&(lat, lng): &(i32, i32)) {
    println!("coordinate lat: {}, lng: {}", lat, lng);
}