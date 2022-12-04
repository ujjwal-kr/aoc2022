use std::fs::File;
use std::io::Read;


pub fn solve() {
    let path = "./day4/input.txt";
    let mut file: File = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let pairs: Vec<&str> = contents.split("\n").collect();
    let mut overlap_count = 0i32;
    let mut overlap_atall = 0i32;

    for pair in pairs {
        let r1: &str = pair.split(",").collect::<Vec<&str>>()[0];
        let r2: &str = pair.split(",").collect::<Vec<&str>>()[1];

        let r1p1: i32 = r1.split("-").collect::<Vec<&str>>()[0].parse().unwrap();
        let r1p2: i32 = r1.split("-").collect::<Vec<&str>>()[1].parse().unwrap();

        let r2p1: i32 = r2.split("-").collect::<Vec<&str>>()[0].parse().unwrap();
        let r2p2: i32 = r2.split("-").collect::<Vec<&str>>()[1].parse().unwrap();

        let mut r1_vec: Vec<i32> = vec![];
        let mut r2_vec: Vec<i32> = vec![];

        for n in r1p1..=r1p2 {
            r1_vec.push(n)
        }
        for n in r2p1..=r2p2 {
            r2_vec.push(n)
        }

        // part 1
        let mut contains: bool = true;
        if r1_vec.len() > r2_vec.len() {
            for r2 in r2_vec.clone() {
                if !r1_vec.contains(&r2) {
                    contains = false
                }
            }
        } else {
            for r1 in r1_vec.clone() {
                if !r2_vec.contains(&r1) {
                    contains = false
                }
            }
        }

        if contains {
            overlap_count += 1;
        }

        // part 2

        let mut overlaps: bool = false;
        if r1_vec.len() > r2_vec.len() {
            for r2 in r2_vec {
                if r1_vec.contains(&r2) {
                    overlaps = true
                }
            }
        } else {
            for r1 in r1_vec {
                if r2_vec.contains(&r1) {
                    overlaps = true
                }
            }
        }

        if overlaps {
            overlap_atall += 1;
        }
    }

    println!("{}, {}", overlap_count, overlap_atall);
}