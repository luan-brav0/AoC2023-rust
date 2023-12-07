use crate::file_reader;

pub fn solve() {
    let data = file_reader::lines_to_vec("day3.txt").unwrap();
    let mut num: String = String::new();
    data.into_iter().enumerate().for_each(|(i, l)| {
        println!("{}: {}", i, l);
        l.chars().enumerate().for_each(|(j, c)| {
            println!("{}: {}", j, c);
            while c.is_numeric() {
                num.push(c);
            }
        });
    })
}
