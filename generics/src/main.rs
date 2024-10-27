// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Czytaj więcej od {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {

// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("oczywiście, jak już zapewne wiecie, ludzie"),
        reply: false,
        retweet: false,
    };
    println!("1 nowy tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Drużyna Penguins wygrała Puchar Stanleya!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "Pittsburgh Penguins po raz kolejny są najlepszą \
    // drużyną hokejową w NHL.",
    //     ),
    // };
    // println!("Dostępny jest nowy artykuł! {}", article.summarize());

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Witaj", y: 'c' };
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("Największa liczba to {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("Największy znak to {}", result);
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

pub fn notify(item: &impl Summary) {
    println!("Z ostatniej chwili! {}", item.summarize());
}
