struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Associated functions --------------------------------
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last name

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 255;

    println!("c Color : {} {} {}", c.red, c.green, c.blue);

    // Struct Update - anything not explictly set, like green and blue, will be set using the green and blue from c.
    // remember k will be initialized with the value of c. If you change the value in c after initialization,
    // it will not have new values. Values is assign during initialization.

    let k = Color { red: 0, ..c };

    println!("k Color : {} {} {}", k.red, k.green, k.blue);

    // can also do this. must use ..c if using all the fields

    let l = Color { ..c };

    println!("l Color : {} {} {}", l.red, l.green, l.blue);

    let d = Color {
        red: 255,
        green: 255,
        blue: 0,
    };

    println!("d Color : {} {} {}", d.red, d.green, d.blue);

    let mut p = Person::new("James", "Bond");

    println!("My Name is {}!", p.get_full_name());

    p.set_last_name("Ingram");

    println!("My New Name is {}!", p.get_full_name());

    println!("My Name in tuple is {:?}!", p.to_tuple());
}
