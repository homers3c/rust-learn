pub fn loops_example(list: [i32; 5]) {
  println!("while loop");
  let mut count = 0; 
  while count < list.len() {
    let element = list[count];
    
    println!("count: {count} | element: {element}");
    count += 1;
  }
 
  println!("for/in loop");
  for element in list {
      println!("the value is: {element}");
  }

  println!("for/in loop in tuple.rev()");
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}