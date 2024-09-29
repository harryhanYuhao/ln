#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

struct Point {
    x: i32,
    y: i32,
}

pub trait Summary {
    fn summarise(&self) -> String {
        String::from("AHA.. Sumarising")
    }
    fn more() -> String {
        String::from("Want more?")
    }
    fn notify_method<T:Summary> (&self: &T) -> String{
        format!("Notification: {}", item.summarise())
    }
    fn notify(item: &impl Summary) -> String{
        format!("Notification: {}", item.summarise())
    }
}

impl Summary for Rect {}

impl Summary for Point {
    fn summarise(&self) -> String {
        format!("Point at ({}: {})", self.x, self.y)
    }
}

fn main() {
    let rect = Rect {
        width: 10,
        height: 20,
    };

    let point = Point { x: 1, y: 2 };

    println!("The area of the rectangle is {}", rect.area());
    dbg!(&rect);
    println!("{:?}", rect);

    let square = Rect::square(3);
    println!("The area of the square is {}", square.area());
    println!("{:?}", Rect::more());
    println!("{}", point.summarise());
    println!("{}", rect.summarise());
    println!("{}", Point::notify(&point));
    println!("{}", Rect::notify(&rect));
}

impl Rect {
    fn area(&self) -> u32 {
        // this is a method
        self.width * self.height
    }

    // an associated function, first param not self
    // call with React::square(2);
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}
