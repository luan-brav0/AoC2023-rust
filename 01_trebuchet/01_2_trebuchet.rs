mod file_reader;

const NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_two_digit_keys(nums: &Vec<String>) -> Vec<i32> {
    return nums
        .iter()
        .map(|s| {
            (format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
}

fn get_digits(line: &str) -> String {
    return line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.is_numeric() {
                return Some(c);
            } else {
                for (j, num) in NUMS.iter().enumerate() {
                    if line[i..].starts_with(num) {
                        return Some(char::from_digit(j as u32, 10).unwrap());
                    }
                }
                return None;
            }
        })
        .collect::<String>();
}

fn find_nums(data: &Vec<String>) -> Vec<String> {
    let mut nums: Vec<String> = Vec::new();
    for line in data {
        nums.push(get_digits(line));
    }
    return nums;
}

fn main() {
    let file_path = "./01_1_trebuchet.txt";
    let data = file_reader::lines_to_vec(file_path).unwrap();

    let nums: Vec<String> = find_nums(&data);
    let keys: Vec<i32> = get_two_digit_keys(&nums);
    let sum: i32 = keys.iter().sum();

    println!("{:?}", find_nums(&data));
    println!("{:?}", keys);
    println!("{}", sum);
}
