use exercises::fibonnaci::fibonnaci;

use crate::exercises::fah_to_cel::convert_fah_to_celcius;

mod codes;
mod exercises;

fn main() {
    let list = [1, 2, 3, 4, 5];

    let celcius_result = convert_fah_to_celcius(80);
    println!("fah to celcius: {celcius_result}");

    let fibonnaci_result = fibonnaci(10);
    println!("fibonnaci result: {fibonnaci_result}");

    println!("[loops_example]");
    codes::loops::loops_example(list);
    
    println!("[list_example]");
    codes::list::list_example(1, list); 
}
