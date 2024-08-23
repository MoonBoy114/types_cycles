

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() {
//     // let number_list =  vec![32,56,11,890,44];
//     // let mut largest = number_list[0];
//     // for number in number_list {
//     //     if number > largest {
//     //         largest = number;
//     //     }
//     // }
//     // println!("Наибольшее число: {}", largest);
//     let number_list = vec![12,12,54,989,55];
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

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point1 = Point {
        x: 5,
        y: 10.4,
    };
    let point2 = Point {
        x: "Привет",
        y: 89,
    };
    let point3 = point1.mixup(point2);
    println!("point3.x = {}, point3.y = {}", point3.x, point3.y);

}