use std::collections::HashMap;
fn main(){
   //This is generalized version of first_ocuuring character
    println!("{:?}",generic_occurring_chars(&"BCBA",2));
}
fn generic_occurring_chars(s:&str,occurring:u8)->Option<char>{
    let mut hashmap =HashMap::new();
    let mut count;
    //This ensures that keys and values are pushed into the hashmap before any operation on the hashmap
    for cha in s.chars(){
      count = hashmap.entry(cha).or_insert(0);
          *count+=1;
    }
   //This won't panic since we only unwrapping after the characters are inserted into hashmap.
    for cha in s.chars(){
    if hashmap.get(&cha).unwrap() == &occurring{
            return Some(cha.clone());
        }
        }
    None
    
}
