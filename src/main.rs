use::std::collections::HashMap;

#[derive(Debug)]
struct Mode {
    number: i32,
    count: i32,
}

fn sum(data: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0 .. data.len() {
        sum += data[i];
    }
    sum
}

fn mean(data: &Vec<i32>) -> i32 {
    sum(data) / data.len() as i32
}

fn median(data: &Vec<i32>) -> i32 {
    let mut sort_data = data.clone();
    sort_data.sort_by(|a, b| a.cmp(b));

    let half_data_length = sort_data.len() / 2;
    if is_even(half_data_length) {
        // pair of middle numbers
        (sort_data[half_data_length - 1] + sort_data[half_data_length]) / 2
    } else {
        sort_data[half_data_length - 1]
    }
}

fn is_even(length: usize) -> bool {
    length / 2 == 0
}

fn mode(data: &Vec<i32>) -> Mode {
    let mut map = HashMap::new();

    for number in data {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode = Mode { number: 0, count: 0 };

    for (key, value) in &map {
        if value > &mode.count {
            mode.number = *key.clone();
            mode.count = value.clone();
        }
    }
    mode
}

fn main() {
    let random_numbers = vec!
    [
        76, 10, 52, 69, 43, 0, 74, 9, 98, 38, 24, 39, 48, 87, 92, 11, 91, 83,
        89, 19, 32, 67, 56, 37, 45, 52, 14, 82, 45, 53, 30, 99, 58, 56, 18,
        61, 100, 80, 27, 80, 61, 70, 91, 75, 93, 17, 37, 82, 11, 71, 28, 8,
        2, 15, 22, 77, 51, 21, 85, 53, 64, 60, 8, 97, 45, 91, 45, 85, 71, 13,
        31, 42, 51, 64, 30, 47, 39, 78, 38, 84, 62, 63, 80, 25, 43, 28, 27, 27,
        75, 21, 64, 1, 42, 85, 82, 14, 14, 67, 65, 72, 76, 94, 45, 11, 47, 70,
        6, 47, 63, 9, 19, 54, 53, 64, 22, 41, 8, 78, 77, 21, 17, 98, 18, 7, 49,
        76, 37, 91, 44, 5, 43, 60, 19, 20, 98, 95, 32, 84, 66, 82, 35, 85, 22,
        36, 95, 34, 56, 21, 11, 79, 35, 12, 21, 93, 62, 28, 33, 10, 26, 58, 4,
        10, 35, 96, 9, 32, 46, 74, 43, 64, 64, 28, 6, 100, 48, 21, 49, 40, 62,
        89, 60, 6, 74, 98, 60, 54, 22, 18, 7, 32, 40, 27, 31, 73, 28, 64, 70,
        34, 85, 63, 68, 53, 40, 23, 46, 69, 30, 93, 87, 22, 80, 33, 18, 45, 15,
        33, 92, 81, 76, 10, 20, 19, 33, 0, 18, 7, 3, 0, 23, 68, 80, 9, 17, 49,
        84, 57, 18, 78, 75, 13, 87, 22, 29, 27, 64, 91, 11, 24, 28, 30, 5, 16,
        86, 34, 54, 45, 95, 86, 88, 24, 74, 79, 52, 71, 55, 75, 35, 94, 16, 40,
        31, 9, 41, 26, 47, 3, 66, 92, 99, 13, 72, 64, 21, 96, 96, 1, 9, 5, 24,
        0, 30, 56, 50, 19, 77, 37, 96, 100, 92, 95
    ];

    println!("The value of mean is {}.", mean(&random_numbers));
    println!("The value of median is {}.", median(&random_numbers));
    let mode = mode(&random_numbers);
    println!("The value of mode is {}. (count: {})", mode.number, mode.count);
}
