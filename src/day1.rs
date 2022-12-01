use std::fs::File;
use std::io::Read;

pub fn solve() {
    let path = "./day1/input.txt";
    let mut file: File = File::open(path).unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        let a: Vec<&str> = contents.split("\n\n").collect();
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
        let mut max = totals.iter().max().cloned().unwrap();
        println!("{}", max);
        let mut t3_vec: Vec<u32> = vec![];
        for _ in 0..3 { 
            t3_vec.push(max);
            let idx = totals.clone().iter().position(|&x| x == max).unwrap();
            totals.remove(idx);
            max = totals.iter().max().unwrap().clone();
        }
        let mut t3_sum = 0u32;
        for i in t3_vec {
            t3_sum = i + t3_sum
        }
        println!("{t3_sum}");

}