#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(50 * scale),
        height: 20,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 1000,
        height: 20,
    };

    dbg!(&rect1);

    let area = rect1.area();
    println!("Pole wynosi {}", area);

    let hold = rect1.can_hold(&rect2);
    println!("{hold}");

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}