use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn solve() {
    let path = "./day3/input.txt";
    let mut file: File = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut i = 0i32;
    for c in b'a'..=b'z' {
        i += 1;
        map.insert(c as char, i);
    }
    for c in b'A'..=b'Z' {
        i += 1;
        map.insert(c as char, i);
    }
    let mut total_shared = 0i32;
    let item_vec = contents.split("\n").collect::<Vec<&str>>();
    for item in item_vec.clone() {
        let p1 = &item[0..item.len()/2];
        let p2 = &item[item.len()/2..item.len()];
        let mut already_shared: Vec<char> = vec![];
        for p1c in p1.chars() {
            for p2c in p2.chars() {
                if p1c == p2c {
                    if !already_shared.contains(&p1c) {
                        total_shared += map.get(&p1c).unwrap()
                    }
                    already_shared.push(p1c);
                }
            }
        }
    }
    println!("{}", total_shared);


    let mut total_i3_shared = 0i32;
    for i3 in item_vec.chunks(3) {
        let p1 = i3[0];
        let p2 = i3[1];
        let p3 = i3[2];

        let mut commons = vec![];
        
        for i in p1.chars() {
            let mut already_shared: Vec<char> = vec![];
            for j in p2.chars() {
                if i == j {
                    if !already_shared.contains(&i) {
                        commons.push(i);
                    }
                    already_shared.push(i);
                }
            }
        }
        
        let mut f: char = 'a';

        for i in p3.chars() {
            let mut already_shared: Vec<char> = vec![];
            for j in commons.clone() {
                if i == j {
                    if !already_shared.contains(&i) {
                        f = i;
                    }
                    already_shared.push(i)
                }
            }
        }

       total_i3_shared += map.get(&f).unwrap()
    }

    println!("{}", total_i3_shared);
}