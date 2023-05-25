# **Chapter 3**

## Exercises ( src/exercises )
1. Convert temperatures between Fahrenheit and Celsius.
2. Generate the nth Fibonacci number.
3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song


## **Variables are immutable by default.**
**Contants**
> Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about

> The last difference is that constants may be set only to a constant expression, not th]result of a value that could only be computed at runtime.

example: 
```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
###

**Shadowing**
> Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

```rs
fn main() {
    let x = 5; // 5

    let x = x + 1; // 6

    { // inner scope
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // 6
}

```
> The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:
```rs
// works because is shadowing
let spaces = "   ";
let spaces = spaces.len();


// compile-error -> because cant change value with different type. 
let mut spaces = "   "; // String
spaces = spaces.len();  // Number 
//  error[E0308]: mismatched types

```

## **Data Types**
### Scalar Types
> A scalar type represents a single value. Rust has four primary scalar types: `integers`, `floating-point numbers`, `Booleans`, and `characters`. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

### Integer types
> An integer is a number without a fractional component. We used one integer type in Chapter 2, the u32 type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

```rs
u32 -> unsigned integer 32bit (only positive) 
i32 -> signed integer 32bit (positive or negative)

Length	 Signed	 Unsigned
8-bit	   i8	    u8
16-bit	   i16	    u16
32-bit	   i32	    u32
64-bit	   i64	    u64   
128-bit	   i128     u128

signed = +30 or +40 -> positive and negative
unsgined = 30 or 40 -> only positive
arch       isize    usize
```
*Signed / Unsigned*
> Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using two’s complement representation.

> Each signed variant can store numbers from `-(2n - 1)` to `2n - 1 - 1` inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from `-(27)` to `27 - 1`, which equals `-128` to `127`. Unsigned variants can store numbers from `0` to `2n - 1`, so a `u8` can store numbers from `0` to `28 - 1`, which equals `0` to `255`.

> Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

> You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

```rs

Number    literals	Example
Decimal	  98_222
Hex	      0xff
Octal	  0o77
Binary	  0b1111_0000
Byte      (u8 only)	b'A'

```

*Integer Overflow* 🟨

Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.

When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the wrapping_* methods, such as wrapping_add.
- Return the None value if there is overflow with the checked_* methods.
- Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
- Saturate at the value’s minimum or maximum values with the saturating_* methods.

### Floating-Point Types
> Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

example
```rs
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

```

> Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

*Numeric operations*
> Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer. The following code shows how you’d use each numeric operation in a let statement:

```rs
// Mathematical Operations:
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```
> Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable. `Appendix B` contains a list of all operators that Rust provides.


### The Boolean Type

> As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:

```rs
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### The Character Type
> Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:

```rs
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. We’ll discuss this topic in detail in “Storing UTF-8 Encoded Text with Strings” in Chapter 8.

### Compound Types
> Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

*The Tuple Type*
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

     let five_hundred = tup.0; // 500 ( index: 0 )

    // The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

    let tup2 = (500, 6.4, 1); 

    let (x, y, z) = tup2;

    println!("The value of y is: {y}"); // 6.4 -> second position in tuple
}
```

### The Array Type
> Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

🟨Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the `vector type`, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector. Chapter 8 discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:
```rs
// fixed length
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let variable: [type, size] = [ele1, ele2];
let a: [i32; 2] = [1, 2];

// You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
let a = [3; 5];
let a = [3, 3, 3, 3, 3]; 

// 🟨 An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

let a = [1,2,3];
let first = a[0]; // 1
let second = a[1];  // 2

```


## Functions
> Functions are prevalent in Rust code. You’ve already seen one of the most important functions in the language: the main function, which is the entry point of many programs. You’ve also seen the fn keyword, which allows you to declare new functions.

🟩 Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:
```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

🟨 Note that we defined another_function after the main function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

### Parameters
> We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

```rs
fn main() {
  another_function(10);
}

fn another_function(value: i32, unit_label: char) {
    println!("the value is: {value}{unit_label");
}
```

### Statements and Expressions
> Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an `expression-based language`, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.


```rs
fn main() {
    let y = 6; // its statement
}
```
Function definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
```rs
fn main() {
    let x = (let y = 6);
}

// When you run this program, the error you’ll get looks like this:
//error: expected expression, found statement (`let`)
//  --> src/main.rs:2:14
//  |
//  |     let x = (let y = 6);
//  = note: variable declaration using `let` is a statement

```
The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:
```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Expression
{
    let x = 3;
    x + 1
}
```

### Functions with Return Values
> Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32. Try running this code;

> The 5 in five is the function’s return value, which is why the return type is i32. Let’s examine this in more detail. There are two important bits: first, the line let x = five(); shows that we’re using the return value of a function to initialize a variable. Because the function five returns a 5, that line is the same as the following:


## Control Flow
> The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

*if Expressions*
> All if expressions start with the keyword if, followed by a condition. In this case, the condition checks whether or not the variable number has a value less than 5. We place the block of code to execute if the condition is true immediately after the condition inside curly brackets. Blocks of code associated with the conditions in `if expressions` are sometimes called `arms`, just like the arms in match expressions that we discussed in the “Comparing the Guess to the Secret Number” section of `Chapter 2`.

It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error. For example, try running the following code:
```rs
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }

  // output:
  //  if number {
  //       ^^^^^^ expected `bool`, found integer
}
```

🟨 Unlike languages such as Ruby and JavaScript, Rust will not `automatically try to convert non-Boolean types to a Boolean`. You `must be explicit` and always provide if with a Boolean as its condition. If we want the if code block to run only when a number is not equal to 0, for example, we can change the if expression to the following:

*Using if in a let Statement*
> Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } 
                 else { 6 };

    println!("The value of number is: {number}"); // 5
}
```

🟨 Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type; in Listing 3-2, the results of both the if arm and the else arm were i32 integers. If the types are mismatched, as in the following example, we’ll get an error:

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");

//  |  let number = if condition { 5 } else { "six" };
//  |                                 -       ^^^^^ expected integer, found `&str`
}
```

### Repetition with Loops
> It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning. To experiment with loops, let’s make a new project called loops.

Rust has three kinds of loops: loop, while, and for. Let’s try each one.

*Repeating Code with `loop`*
> he loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rs
fn main() {
    loop {
        println!("again!");
    }
}
```

*Returning Values from Loops*
```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // its return
        }
    };

    println!("The result is {result}"); // 20
}
```

*Loop Labels to Disambiguate Between Multiple Loops ( inner loops )*
> If you have **loops within loops**, *break* and `continue` apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:
```rs

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// outputs:
// count = 0
// remaining = 10
// remaining = 9
// count = 1
// remaining = 10
// remaining = 9
// count = 2
// remaining = 10
// End count = 2
```

### Conditional Loops with `while`
> A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a while loop. In Listing 3-3, we use while to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

```rs

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer. While a condition evaluates to true, the code runs; otherwise, it exits the loop.


### Looping Through a Collection with for 
> You can choose to use the while construct to loop over the elements of a collection, such as an array. For example, the loop in Listing 3-4 prints each element in the array a.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // or index < a.len()
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```


