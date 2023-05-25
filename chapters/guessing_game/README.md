### Chapter 2
- 


## Extras
**if-else statement**
- Its works as others language, but the parentheses is not required.
- You can use `let` to get the returns from `if-else statement`

*Enumaration `Result<>` - Chapter 6*
> Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

*Reference arguments `&`*
> The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

**Types of imports**
*Associated functions*
> Function that is implemented on type, ex: `String::new` -> _new_ is a associated function implemented on String.

**How loops works?**

*basic loop*
> The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one
```rs

  fn main() {
    loop {
      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("fail on try get line");

      let guess_size = guess.len();

      if guess_size < 4 {
        println!("is lower than 4 -> break");
        break
      } else if guess_size < 8 {
        println!("is lower than 8 -> continue");
        continue
      }

      println!("does not match any if-else statement case");
    }
  }
```

*nested loops -> break/continue*
> It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
```rs
  fn main() {
    'outer: loop {
      println!("'other loop -> first")

      'inner: loop {
        println("Entered the inner loop")

        break 'outer;
      }
      println!("This point will never be reached"); 
    }
    
    println!("Exited the outer loop in 'inner loop");
  }
```
*returning the loop content with `break`*
> One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break, and it will be returned by the loop expression.
```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

