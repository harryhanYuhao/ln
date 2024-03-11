fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

static S: &'static str = "static";

fn main() {
    let string1 = String::from("abc");    
    let string2 = String::from("abcd");    
    println!("{}", longest(&string1, &string2));
    println!("{}", S);
}
