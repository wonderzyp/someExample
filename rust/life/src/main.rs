



fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");

    // {
    //     let string2 = String::from("short");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longger is {}", result);
    // }
    
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longger is {}", result);
}
