pub fn list_example(index: usize, list: [i32; 5]) {
  if index >= list.len() {
      panic!("out of index")
  }

  let element_in_list = list[index];

  println!("you choose the index {index} and found element {element_in_list}")
}