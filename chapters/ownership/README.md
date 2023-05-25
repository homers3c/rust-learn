# Chapter 4

content: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation

### Topics
- Ownership
- Stack and Heap

## Ownership 🟥
> Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. features: borrowing, slices, and how Rust lays data out in memory.

**What Is Ownership?**

Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: `memory is managed through a system of ownership with a set of rules that the compiler checks`. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you’ll find it to naturally develop code that is safe and efficient. Keep at it!

When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique. In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.


**The Stack and the Heap** 🟥

In languages like Rust, understanding the role of the stack and the heap is crucial as it impacts language behavior and decision-making. `Both are memory regions available for use during runtime`, but they differ in structure.

🟨 The `stack` operates on a `last-in`, `first-out` basis, storing values in the order they are received and removing them in reverse order. This arrangement is akin to a stack of plates where you add or remove plates from the top. *The stack necessitates data of known, fixed size*. *If data size is unknown or mutable at compile time, it needs to be stored in the heap.*

🟨 The `heap`, being less organized, assigns space when requested. *The memory allocator identifies a suitable spot, marks it as in-use, and returns a pointer to that location*, `a process termed heap allocation`. The pointer can be stored on the stack, but accessing the actual data requires following the pointer.

Stack pushing is quicker than heap allocation as the allocator doesn't need to search for storage space. In contrast, heap allocation involves finding sufficient space and performing administrative tasks for future allocations. Accessing heap data is slower than stack data as it necessitates pointer-following.

When a function is called, its input values and local variables are pushed onto the stack and popped off when the function ends.

Ownership in Rust deals with monitoring heap data usage, minimizing duplicate heap data, and clearing unused heap data to save space. Understanding ownership alleviates the need to focus on the stack and heap, but it's important to know that its primary function is to manage heap data.

**Ownership Rules**
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

**Variable Scope** 🟨

As a first example of ownership, we’ll look at the scope of some variables. A scope is the range within a program for which an item is valid. Take the following variable:

```rs
{
  // "var1" is not valid here, it’s not yet declared
  let var1 = "hello"; // is valid from this point forward
  // do something with "var1"
} // this scope is now over, and "var1" is no longer valid.
```

The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. *The variable is valid from the point at which it’s declared until the end of the current scope*. Listing 4-1 shows a program with comments annotating where the variable s would be valid.

- When "var1" comes into scope, it is valid.
- It remains valid until it goes out of scope.

**The String Type**

The concept of `ownership` in Rust is illustrated using more complex data types, such as the String type, that are stored on the *heap*. `Rust automatically manages the cleanup of heap data`, which is important for types like String that can store an unknown amount of text at compile time.

String literals, which are hardcoded into the program, are immutable and not always suitable for scenarios requiring text use, like storing user input. For such cases, Rust provides the String type. A String can be created from a string literal using the from function:

```rs
let s = String::from("hello");
```

The double colon :: operator is used to namespace this specific from function under the String type.

Unlike string literals, String type can be mutated:

```rs
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```

The key difference between String and string literals is their memory handling. String can be mutated because it manages heap-allocated memory, allowing it to accommodate changes in the data it stores.

**Memory and Allocation** 🟥
n the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

🟨 With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

🟨However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:
```rs
{
 let var1 = String::from("hello"); // "var1" is valid from this point forward
 // do stuff with "var1"
       
} // this scope is now over, and "var1" is no longer valid
```

🟨 There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

> Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

This pattern has a profound impact on the way Rust code is written.

**Variables and Data Interacting with Move** 🟥
Multiple variables can interact with the same data in different ways in Rust. Let’s look at an example using an integer in Listing 4-2
```rs
let x = 5;
let y = x;
```
🟨 We can probably guess what this is doing: “*bind* the value *5* to `x`; then make a copy of the value in `x` and *bind* it to `y`.” We now have two variables, `x` and `y`, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

String version:
```rs
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar, so we might assume that the way it works would be the same: that is, the second line would make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens.

🟨  A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

```md
String struct -> stored on the stack: (left)
1. Pointer to the memory that holds contents ( on the heap )
2. Length
3. Capacity

String content -> store on the heap (right)
```
<img width="400" src="./assets/trpl04-01.svg" />

*The `length` is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.*

*When we `assign "s1" to "s2"`, the String data is copied, meaning we `copy the pointer, the length, and the capacity that are on the stack`. **We do not copy the data on the heap that the pointer refers to**. In other words, the data representation in memory looks like.

<img width="400" src="./assets/trpl04-02.svg" />

🟥 *Pointing to the same reference in memory heap, because the when assign s1 to s2 only copy the string config on the stack, pointer reference, length and capacity.*

The representation does not look like Figure 4-3, which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.

<img width="400" src="./assets/trpl04-03.svg" />

🟥 Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows **both data pointers pointing to the same location**. This is a problem: when `s2` and `s1` go out of scope, they will **both try to free the same memory**. This is known as a **double free error** and is one of the **memory safety bugs** we mentioned previously. **Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.**

To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:

```rs
let s1 = String::from("hello");
let s2 = s1;
// error:
// |   ^^ value borrowed here after move

println!("{}, world!", s1);
```

🟥 If you’ve heard the terms **shallow copy** and **deep copy** while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because **Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move**. In this example, we would say that s1 was moved into s2. So, what actually happens is shown in Figure 4-4.

<img width="400" src="./assets/trpl04-04.svg" />

That solves our problem! With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

**Variables and Data Interacting with Clone**

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before

```rs
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This works just fine and explicitly produces the behavior shown in Figure 4-3, where the heap data does get copied.

When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.

🟨 **Stack-Only Data: Copy**

There’s another wrinkle we haven’t talked about yet. This code using integers—part of which was shown in Listing 4-2—works and is valid:

```rs
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

But this code seems to contradict what we just learned: we don’t have a call to clone, but `x` is still valid and wasn’t moved into `y`.

🟨 The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

🟨 Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see `“Derivable Traits”` in Appendix C.

So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

🟥 **Ownership and Functions**

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.

```rs
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn main() {
  let s = String::from("hello");  // s comes into scope

  takes_ownership(s);             // s's value moves into the function...
                                  // ... and so is no longer valid here

  let x = 5;                      // x comes into scope

  makes_copy(x);                  // x would move into the function,
                                  // but i32 is Copy, so it's okay to still
                                  // use x afterward
} // Here, "x" goes out of scope, then s. But because s's value was moved, nothing special happens.
```

If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

🟥 **Return Values and Scope**

Returning values can also transfer ownership. Listing 4-4 shows an example of a function that returns some value, with similar annotations as those in Listing 4-3.

```rs

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
                                        // - moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

Rust does let us return multiple values using a tuple, as shown in Listing 4-5

```rs
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
