use std::io;

fn add_item_in_reference(var1: &mut Vec<String>) {
    var1.push("the new".to_string());
}

fn input(text: &str) -> Result<String, String> {
    println!("{text}");
    let mut content = String::new();

    match io::stdin().read_line(&mut content) {
        Ok(_) => Ok(content),
        Err(_) => Err("fail on try input text".to_string())
    }
}

fn start_loop() -> Vec<String>{
    let mut list: Vec<String> = vec![];

    return loop {
        match input("type something: ") {
            Ok(res) => match res.as_str().trim() {
                "stop" => {
                    break list;
                },
                _ => {
                    let content = res.to_string();
                    list.push(content);
                }
            },
            Err(_) => {}
        }
    };
}

fn main() {
    let mut list = start_loop();
    for element in list.clone().into_iter() {
        match element.as_str().trim() {
            "pedro" => println!("found: pedro"),
            _ => println!("found: random") 
        }
    }

    println!("size: {}", list.len()); // current size
    add_item_in_reference(&mut list); // send reference and add one element
    println!("new size: {}", list.len()) // size changed by drop_variable
}

// playing with =>  loops / ownership / reference / match