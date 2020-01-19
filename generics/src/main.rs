#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// NOTE: `impl` also needs the generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    // step 1
    code_duplication();

    // step 2
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("[2] The largest number is {}", result);

    // step 3
    let char_list = vec!['y', 'm', 'a', 'q', '#'];
    let result = largest_char(&char_list);
    println!("[3] The largest char is {}", result);

    // step 4
    let result = largest(&number_list);
    println!("[4] The largest number is {}", result);

    let result = largest(&char_list);
    println!("[4] The largest char is {}", result);

    // step 5
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("[5] {:?}", integer_point);
    println!("[5] {:?}", float_point);

    // step 6
    println!("[6] integer_point.x = {}", integer_point.x());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn code_duplication() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("[1] The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("[1] The largest number is {}", largest);
}
