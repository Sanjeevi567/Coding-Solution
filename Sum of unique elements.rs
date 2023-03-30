use std::collections::HashSet;
use core::hash::Hash;
use std::iter::Sum;
fn main(){
//Simpler version
  let m=HashSet::from([1,2,1,2]);
  println!("{}",m.iter().sum::<usize>());
   
   let neg = [-5,-6,-6,-5,-2];
   let pos = [1,2,2,4,5,67];
   
 println!("{:?}",sum_of_unique_elements(neg));  
 println!("{:?}",sum_of_unique_elements(pos));  
}
//Generic Vesion but lot involved 
fn sum_of_unique_elements<const N:usize,T>(s:[T;N])->isize
where isize: Sum<T>,
T:Ord + Clone + Hash,
{
    let m=HashSet::from(s);
    m.iter().cloned().sum()
}
