// File modules
// use std::fs::File;
// use std::io::prelude::*;

// Command line modules
// use std::env;

// Read user input modules
// use std::io;

// HashMap modules
// use std::collections::HashMap;

// Include random functions
// extern crate rand;
// use rand::Rng;

// Include a module
// mod my_lib;

// Include regex modules
// extern crate regex;
// use regex::Regex;

// Define a module
// mod my_mod {
//     fn chicken() {
//         println!("Chicken");
//     }

//     pub fn print_message() {
//         chicken();
//         println!("How it's going");
//     }

//     // Nested module
//     pub mod water {
//         pub fn print_message() {
//             println!("I'm water");
//         }
//     }
// }

// const MAXIMUM_NUMBER: u8 = 20;

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right
// }

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// struct ColorTuple(u8, u8, u8);

// struct Rectangle {
//     width: u8,
//     height: u8
// }

// impl Rectangle {
//     fn print_description(&self) {
//         println!("Rectangle: {} x {}", self.width, self.height);
//     }

//     fn is_square(&self) -> bool {
//         self.width == self.height
//     }
// }

// struct Person {
//     name: String,
//     age: u8
// }

// impl ToString for Person {
//     fn to_string(&self) -> String {
//         return format!("My name is {} and I am {}", self.name, self.age);
//     }
// }

// trait HasVoiceBox {
//     // Speak
//     fn speak(&self);
//     // Check if can speak
//     fn can_speak(&self) -> bool;
// }

// impl HasVoiceBox for Person {
//     fn speak(&self) {
//         if self.can_speak() {
//             println!("Hello! My name is {}", self.name);
//         } else {
//             println!("...");
//         }
//     }

//     fn can_speak(&self) -> bool {
//         if self.age > 0 {
//             return true;
//         } return false;
//     }
// }

fn main() {
    // # Iterate through an array
    // let animals = vec!["Rabbit", "Cat", "Rat"];

    // for (index, a) in animals.iter().enumerate() {
    //     println!("{}: {}", index, a);
    // }

    // # Enum
    // let player_direction: Direction = Direction::Up;

    // match player_direction {
    //     Direction::Up => println!("We are going up"),
    //     Direction::Down => println!("We are going down"),
    //     Direction::Left => println!("We are going left"),
    //     Direction::Right => println!("We are going right"),
    // }

    // # Using constant
    // for n in 1..MAXIMUM_NUMBER {
    //     println!("{}", n);
    // }

    // # Tuple
    // let tup1 = (20, 25.1, "Str", false);
    // println!("{}", tup1.2);

    // # Nested tuple
    // let tup1 = (20, 25.1, "Str", false, (1,4,7));
    // println!("{}", (tup1.4).1);

    // # Destruct a tuple
    // let tup2 = (10, 20, 30);
    // let (a,b,c) = tup2;
    // println!("{}", a);
    // println!("{}", b);
    // println!("{}", c);

    // # Function
    // print_numbers_to(10);

    // # References
    // let mut x = 10;
    // let dom = &mut x;
    // *dom += 1;
    // println!("x is {}", x);

    // # Struct
    // let bg = Color { red: 255, green: 70, blue: 15  };
    // println!("{} {} {}", bg.red, bg.green, bg.blue);

    // let mut bg2 = Color { red: 1, green: 1, blue: 1};
    // bg2.blue = 20;
    // println!("{} {} {}", bg2.red, bg2.green, bg2.blue);

    // # Tuple Struct
    // let red = ColorTuple(255, 0, 0);
    // println!("{} {} {}", red.0, red.1, red.2);

    // # Pass by reference
    // let blue = Color { red: 0, green: 0, blue: 255 };
    // print_color(&blue);

    // # Array
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // for n in numbers.iter() {
    //     println!("{}", n)
    // }
    // for i in 1..numbers.len() {
    //     println!("{}", numbers[i]);
    // }

    // let numbers_with_type: [i32; 5] = [1, 2, 3, 4, 5];

    // let repeated_numbers = [2; 400];
    // for n in repeated_numbers.iter() {
    //     println!("{}", n);
    // }

    // # impl
    // let my_rect = Rectangle { width: 10, height: 10 };
    // my_rect.print_description();
    // println!("Is square {}", my_rect.is_square());

    // # String
    // let mut my_string = String::from("Hello, My name is TK.");

    // // check length
    // println!("Length: {}", my_string.len());

    // // check empty
    // println!("Is empty ?: {}", my_string.is_empty());

    // // split white space
    // for token in my_string.split_whitespace() {
    //     println!("{}", token);
    // }

    // split
    // let my_string_with_seperator = String::from("Leave+a+like+of+you+enjoyed!");
    // let tokens: Vec<&str> =my_string_with_seperator.split("+").collect();
    // println!("{}", tokens[2]);

    // trim
    // let my_string_with_spaces = String::from("      My name is Huzen        \n");
    // println!("Before trim {}", my_string_with_spaces);
    // println!("After trim {}", my_string_with_spaces.trim());

    // chars - get character at index
    // match my_string.chars().nth(4) {
    //     Some(c) => println!("Character at index 4 is {}", c),
    //     None => println!("No character at index 4")
    // }

    // // contains
    // println!("Does the string contains TK ?: {}", my_string.contains("TK"));

    // // push string
    // my_string.push_str("I'm a programmer");
    // println!("{}", my_string);

    // replace
    // println!("After replace {}", my_string.replace("TK", "Huzen"));

    // lines
    // let string_with_lines = String::from("The weather is\nnice\noutside mate!");

    // for line in string_with_lines.lines() {
    //     println!("{}", line);
    // }

    // # Trait
    // let tk = Person { name: String::from("TK"), age: 25 };
    // println!("{}", tk.to_string());

    // # Vector
    // let mut my_vector: Vec<i32> = Vec::new();
    // my_vector.push(1);
    // my_vector.push(2);
    // my_vector.push(3);
    // my_vector.remove(0);
    // for n in my_vector.iter() {
    //     println!("{}", n);
    // }

    // let my_vector2 = vec![1,2,3,4];
    // for n in my_vector2.iter() {
    //     println!("{}", n);
    // }

    // # Reading a file
    // let mut file = File::open("info.txt").expect("Can't open the file");

    // let mut content = String::new();
    // file.read_to_string(&mut content)
    //     .expect("Can't read file");

    // println!("File content \n \n {}", content);

    // # Writing a file
    // let mut file = File::create("output.txt").expect("Could not create file");
    // file.write_all(b"Welcome to Rust").expect("Cannot write to the file");

    // # Command line
    // > cargo run arg1 arg2
    // let args: Vec<String> = env::args().collect();

    // for arg in args.iter() {
    //     println!("{}", arg);
    // }

    // # Trait
    // let person = Person { name: String::from("TK"), age: 25 };
    // println!("Can {} speak {}", person.name, person.can_speak());
    // person.speak();

    // let baby = Person { name: String::from("Baby"), age: 0 };
    // println!("Can {} speak {}", baby.name, baby.can_speak());
    // baby.speak();

    // # Match operation
    // let number = 9;

    // match number {
    //     1 => println!("Is is one"),
    //     2 => println!("There is two of them"),
    //     3..=9 => println!("It is between 3 to 9"),
    //     10 | 11 => println!("It is either 10 or 11"),
    //     _ => println!("It doesn't match")
    // }

    // # Reading user input
    // let mut input = String::new();

    // println!("Hey! say something: ");

    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
    //         println!("Success! You said {}", input);
    //     },
    //     Err(e) => println!("Oops! something went wrong {}", e)
    // }

    // # HashMap
    // let mut marks = HashMap::new();

    // // Add values
    // marks.insert("Rust programming", 50);
    // marks.insert("Web development", 70);
    // marks.insert("UX design", 45);
    // marks.insert("Professtional computing", 20);

    // // Find length of HashMap
    // println!("How many subjects have you study ? {}", marks.len());

    // // Get a value
    // match marks.get("UX design") {
    //     Some(mark) => println!("You got for UX Design {} marks", mark),
    //     None => println!("You didn't study UX design")
    // }

    // // Remove a value
    // marks.remove("Web development");

    // // Loop through HashMap
    // for (subject, mark) in &marks {
    //     println!("For {} you got {}", subject, mark);
    // } 

    // // Check for a value
    // println!("Did you study C++ ? {}", marks.contains_key("C++ Programming"));

    // # Random - Need to include the "rand" crate on Cargo.toml 
    // let random_number = rand::thread_rng().gen_range(1, 11); // 1 - 10
    // println!("The random number is {}", random_number);

    // // Flip a coin
    // let random_bool = rand::thread_rng().gen_weighted_bool(2); // 1 of "2" to get true
    // println!("Random boolean: {}", random_bool);

    // # Modules
    // my_lib::print_message();

    // my_mod::print_message();
    // my_mod::water::print_message();

    // # Regex - Need to include the "regex" crate on Cargo.toml
    // let re = Regex::new(r"\w{5}").unwrap(); // if there are 5 letters
    // let text = "test";

    // println!("Found match? {}", re.is_match(text));

    // let re2 = Regex::new(r"(\w(5))").unwrap();
    // match re2.captures(text) {
    //     Some (caps) => println!("Found match {}", caps.get(0).unwrap().as_str()), // or &caps[0]
    //     None => println!("Could not find match")
    // }

    // # Option (Enum)
    // let name = String::from("Huzen Karode");
    // println!("Character at index 1: {}", match name.chars().nth(1) {
    //     Some (c) => c.to_string(),
    //     None => "No character at index 1".to_string()
    // })

    // println!("Occupation is {}", match get_occupation("TK") {
    //     Some (o) => o.to_string(),
    //     None => "No occupation".to_string()
    // })

    


}

// fn print_numbers_to(num: u32) {
//     for n in 1..num {
//         if is_even(n) {
//             println!("{} is even", n);
//         } else {
//             println!("{} is odd", n);
//         }
//     }
// }

// fn is_even(num: u32) -> bool {
//     return num % 2 == 0;
// }

// fn print_color(c: &Color) {
//     println!("{} {} {}", c.red, c.green, c.blue);
// }

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Huzen" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}