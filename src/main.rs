struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 50,
        length: 40,
    };

    println!(" The width is {}",rect1.width );
    println!(" The length is {}",rect1.length );
    println!(" The area is {}",rect1.area() );

}
