fn main() {
    let x = vec![1,2,3];
    let y = Box::new(x);
    
    for i in y.iter(){
        println!("{}", i);
    }
}
