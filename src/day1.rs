use std::fs::File;
use std::io::Read;
use std::io;

pub fn solve() {
    let path = "./day1/input.txt";
    let mut file: File = File::open(path).unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut a: Vec<&str> = contents.split("\n\n").collect();
        let mut totals: Vec<u32> = vec![];
        for e in a {
            let mut total_cal = 0u32;
            let calvec: Vec<&str> = e.split("\n").collect();
            for cal in calvec {
                let cal_num: u32 = cal.trim().parse().unwrap();
                total_cal = total_cal + cal_num
            }
            totals.push(total_cal);
        }
        let max = totals.iter().max().unwrap();
        println!("MAX: {}", max);

        

}