mod file_reader;

fn main() {
    let file_path = "./01_trebuchet.txt";
    let data = file_reader::read_lines(file_path).unwrap();


    let nums: Vec<String> = data
        .iter()
        .map(|s| s
            .chars()
            .filter(|c| c.is_numeric()).collect::<String>()
        )
        .collect::<Vec<String>>();
    
    let keys: Vec<i32> = nums
        .iter()
        .map(|s| 
            (format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap())).parse::<i32>().unwrap()
        )
        .collect::<Vec<i32>>();


    let sum: i32 = keys
        .iter()
        .sum();

    println!("{:?}", nums);
    println!("{:?}", keys);
    println!("{}", sum );
}
