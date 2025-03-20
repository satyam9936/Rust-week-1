use chrono::{Date, DateTime, Local, Utc};

fn main() {
    let utc: DateTime<Utc>=Utc::now();
    let local_time:DateTime<Local>=Local::now();
print!("{}",utc)
}

pub fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![String::from("Harkirat"), String::from("Singh")];
    let v3 = vec![1.0, 2.0, 3.0];
    println!("{}", first_element(v.clone()).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());

    println!("{}", does_exist(v, 1));
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0);
}

fn does_exist<T: std::cmp::PartialEq>(v: Vec<T>, element: T) -> bool {
    let mut iter = v.iter();
    while let Some(value) = iter.next() {
        if *value == element {
            return true;
        }
    }
    return false;
}

//Biggest element
fn biggest_element<T: Ord>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}
//genric over structs
struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    pub fn area(&self) -> T {
        return self.height * self.width
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20
    };

    println!("{}", r.area());
}
struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    pub fn area(&self) -> T {
        return self.height * self.width
    }
}

