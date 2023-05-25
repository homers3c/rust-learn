pub fn fibonnaci(nth: usize) -> usize {
  if nth < 1 {
    0
  } else if nth <= 2 {
    1
  } else {
    fibonnaci(nth - 1) + fibonnaci(nth - 2)
  }
  // let mut elements: Vec<usize> = vec![];
  // let mut index = 0;
  // while index < nth {
  //   let first_element = match elements.get(index - 1) {
  //     Some(res) => res,
  //     _ => &1
  //   };

  //   let second_element = match elements.get(index - 2) {
  //     Some(res) => res,
  //     _ => &0
  //   };

  //   let result = first_element + second_element; 
  //   elements.push(result);

  //   index += 1;
  // }
}