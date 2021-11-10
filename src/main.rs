use std::io;
use rand::{thread_rng, Rng};

fn curse(input: &mut String, craziness: i32) -> String{
    let cursed_chars = vec!['\u{338}', '\u{302}', '\u{35b}', 
                            '\u{351}', '\u{31a}', '\u{33d}', 
                            '\u{308}', '\u{30b}', '\u{311}', 
                            '\u{341}', '\u{344}', '\u{307}', 
                            '\u{30f}', '\u{35d}', '\u{350}', 
                            '\u{30c}', '\u{309}', '\u{306}', 
                            '\u{34b}', '\u{32c}', '\u{355}', 
                            '\u{319}', '\u{339}', '\u{31f}', 
                            '\u{331}', '\u{32f}', '\u{327}', 
                            '\u{32b}', '\u{318}', '\u{35c}', 
                            '\u{32e}', '\u{347}', '\u{345}', 
                            '\u{34e}', '\u{356}', '\u{321}', 
                            '\u{320}', '\u{323}', '\u{33c}', 
                            '\u{33a}', '\u{332}', '\u{334}', 
                            '\u{305}', '\u{31b}', '\u{357}', 
                            '\u{360}', '\u{30a}', '\u{33e}', 
                            '\u{30d}', '\u{340}', '\u{34c}', 
                            '\u{310}', '\u{301}', '\u{346}', 
                            '\u{304}', '\u{300}', '\u{352}', 
                            '\u{30e}', '\u{34a}', '\u{317}', 
                            '\u{31d}', '\u{328}', '\u{32d}', 
                            '\u{348}', '\u{333}', '\u{330}', 
                            '\u{31e}', '\u{34d}', '\u{322}', 
                            '\u{316}', '\u{359}', '\u{353}', 
                            '\u{337}', '\u{343}', '\u{314}', 
                            '\u{342}', '\u{329}', '\u{35a}', 
                            '\u{32a}', '\u{315}', '\u{358}', 
                            '\u{303}', '\u{313}', '\u{312}', 
                            '\u{31c}', '\u{349}', '\u{335}', 
                            '\u{33f}', '\u{354}', '\u{324}', 
                            '\u{325}', '\u{33b}', '\u{326}', '\u{336}'];
    let mut result = String::new();
    for letter in input.chars(){
        result.push(letter);
        for _ in 0..craziness{
            let rand_num = thread_rng()
                        .gen_range(0..cursed_chars.len());
            result.push(cursed_chars[rand_num]);
        }
    }
    result
}

fn main() {
    println!("Enter a string to curse");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read string");

    println!("Enter the craziness level(number)");
    let mut craziness = String::new();
    io::stdin()
        .read_line(&mut craziness)
        .expect("Unable to read string");
    
    match craziness.trim().parse::<i32>(){
        Ok(value) => println!("{}", curse(&mut input, value)),
        Err(e) => println!("{}", e)
    }
}
