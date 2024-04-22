use std::collections::HashMap;
fn main() {
    let string = "abbaab";
    let result = first_occurring_char(string);
    match result {
        Some(c) => println!("The fisrt occurring character is  {:#?}", c),
        None => println!("There is no occurring character in the given string"),
    }
}
fn first_occurring_char(s: &str) -> Option<char> {
    let mut hashmap = HashMap::with_capacity(20);

    for char in s.chars() {
        if hashmap.contains_key(&char) {
            return Some(char.clone());
        } else {
            hashmap.entry(char).or_insert(1);
        }
    }
    None
}
