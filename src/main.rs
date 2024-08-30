use types_cycles::*;
use std::fmt::Display;

// struct  Pair<T> {
//     x: T,
//     y: T,
// }
//  impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//         x,
//         y,
//         }
//     }
//  } 
//  impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("Наибольший член равен x: {}", self.x);
//         } else {
//             println!("Наибольший член равен y: {}", self.y);
//         }
//     }
//  }
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     }
//     else  {
//         y
//     }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn ann_and_return_part(&self, ann: &str) -> &str {
        println!("Пожалуйста, внимание: {}", ann);
        self.part
    }
}
fn main() {
    // let string1 = String::from("xyz");
    // {
    //     let string2 = String::from("abcd");
    //     let result = longest(&string1.as_str(), &string2.as_str());
    //     println!("Самая длинная строка равна: {}", result);
    // }
    let x = 5;
    let novel = String::from("Зовите меня, как хотите. Несколько лет тому назад...");
    let first_sentence = novel.split('.').next().expect("Не смог отыскать '.'");
    let i = ImportantExcerpt {part: first_sentence};
    

}


// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() { 
//     let number_list = vec![12,12,54,989,55];
//     let result = largest(&number_list);
//     println!("наибольшее число равно: {}", result);
//     let char_list = vec!['d', 's', 'a', 'j'];
//     let lagr = largest(&char_list);
//     println!("Наибольший символ равен: {}", lagr);
// }
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T  {
//         &self.x
//     }
// }
// impl Point<f64> {
//     fn dist_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2).sqrt())
//     }
// }
// struct Point2<T, U>{
//     x: T,
//     y: U,
// }
// fn main() {
//     let integer = Point {
//         x: 12,
//         y: 14,
//     };
//     let floa = Point {
//         x: 12.0,
//         y: 11.0,
//     };
//     let result = Point2 {
//         x: 12,
//         y: 9.0,
//     };
//     println!("integer.x = {}", integer.x());
//     println!("Расстояние между точками: {}", floa.dist_from_origin());
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// fn returns_summarize() -> impl Summary {
//     Tweet {
//         username: String::from("Jason"),
//         content: String::from("Youtube"),
//         reply: false,
//         retweet: false,
//     }
// }
// fn main() {
//     // let point1 = Point {
//     //     x: 5,
//     //     y: 10.4,
//     // };
//     // let point2 = Point {
//     //     x: "Привет",
//     //     y: 89,
//     // };
//     // let point3 = point1.mixup(point2);
//     // println!("point3.x = {}, point3.y = {}", point3.x, point3.y);
//     // let integer = Some(5);
//     // let float = Some(5.0);
//     let article = NewsArticle {
//         headline: String::from("Пингвины выигрывают Кубок Стэнли!"),
//         location: String::from("Питтсбург, шт. Пенсильвания, США"),
//         author: String::from("Айсбург"),
//         content: String::from("Питтсбург Пингвинз снова является лучшей 
//  хоккейной командой в НХЛ."),
//     };
//     let tweet = Tweet {
//         username: String::from("Jason"),
//         content: String::from("Youtube"),
//         reply: false,
//         retweet: false,
//     };
//     println!("Tweet: {}", tweet.summarize());
//     // println!("Есть новая статья! {}", article.summarize());
// }