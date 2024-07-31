#[derive(Clone)]
struct Person {
    name: String,
    alter: u32,
    size: Option<f32>,
}

#[derive(Clone)]
struct Konten {
    ingdiba: u128,
    spaskasse: u128,
}

fn main() {
    println!("Hello, world!");
    let mut mony = Konten {
        ingdiba: 120,
        spaskasse: 300,
    };
    let mut alpha = Person {
        name: "apha".to_string(),
        alter: 17,
        size: None,
    };
    println!("{}", size_of::<Person>());
    println!("age: {}", alpha.clone().birthday());
    println!("new age: {}", alpha.alter);

    println!("sum: {}", mony.sum());
    println!("dec: {}", mony.sum());
    println!("dec: {}", mony.sum());
}

impl Person {
    fn birthday(&mut self) -> bool {
        self.alter += 1;
        self.alter < 18
    }
}

impl Konten {
    fn sum(&self) -> u128 {
        self.ingdiba + self.spaskasse
    }
}
