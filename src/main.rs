struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

}

fn main() {

    let rect1 = Rectangle { width: 50, length: 40, };
    let rect2 = Rectangle { width: 45, length: 38, };
    let rect3 = Rectangle { width: 45, length: 48, };

    println!(" The width is {}",rect1.width );
    println!(" The length is {}",rect1.length );
    println!(" The area is {}",rect1.area() );
    println!(" Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2) );
    println!(" Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3) );
}
