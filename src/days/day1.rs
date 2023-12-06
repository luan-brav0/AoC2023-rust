use crate::file_reader;

pub fn solve() {
    let data = file_reader::lines_to_vec("day1.txt").unwrap();

    let nums: Vec<String> = data
        .iter()
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>())
        .collect::<Vec<String>>();

    let keys: Vec<i32> = nums
        .iter()
        .map(|s| {
            (format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    let sum: i32 = keys.iter().sum();

    println!("{:?}", nums);
    println!("{:?}", keys);
    println!("{}", sum);
}
