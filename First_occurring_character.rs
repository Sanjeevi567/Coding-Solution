use std::collections::HashMap;
fn main(){
    let string = "abcdefgha";
    //calling unwrap direclty on result will panic when None variant is returned
    let result =first_occurring_char(string); 
    match result{
        Some(c) => println!("The fisrt occurring character is  {:#?}",c),
        None =>println!("There is no occurring character in given string"),
    }
}
//This function is falliable when No matching is found,that's why we use option type
fn first_occurring_char(s:&str)->Option<char>{ 
  //as long as the string length not exceed 20 characters ,there is no //allocation is needed.
    let mut hashmap=HashMap::with_capacity(20); 
    
   //giving iterator over each character in borrowed string.
  
    for cha in s.chars(){ 
      if hashmap.contains_key(&cha){
  
        //Copy type are not expensive to clone.
          return Some(cha.clone());
      }
      else{
       
         //Only first time it will execute
          hashmap.entry(cha).or_insert(1);
    
      }
    }
    None
} //At this point the hashmap is deallocated since only needed as temporary data structure for efficiency
