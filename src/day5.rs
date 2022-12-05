use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file: File = File::open("./day5/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut map: HashMap<i32, Vec<String>> = HashMap::new();
    let mut map2: HashMap<i32, Vec<String>> = HashMap::new();
    for i in 1..=9 {
        map.insert(i, Vec::<String>::new());
    }
    for i in 1..=9 {
        map2.insert(i, Vec::<String>::new());
    }
    for line in contents.split("\n").collect::<Vec<&str>>() {
        let mut v: Vec<&str> = vec![];
        for c in line.split(" ").collect::<Vec<&str>>() {
            v.push(c);
        }
        if v.len() > 2 {
            let mut skip: bool = false;
            if v.len() > 8 {
                if v[1] == "1" && v[4] == "2" && v[7] == "3" {
                    skip = true
                }
            }
            if !skip {
                match v[0] {
                    "move" => {
                        let num: usize = v[1].parse().unwrap();
                        let src: i32 = v[3].parse().unwrap();
                        let dest: i32 = v[5].parse().unwrap();

                        let mut src_vec = map.get(&src).unwrap().clone();
                        let mut dest_vec = map.get(&dest).unwrap().clone();
                        for _ in 0..num {
                            let d: Vec<String> = src_vec.drain(..1).collect();
                            dest_vec.splice(..0, d);
                        }
                        map.insert(src, src_vec);
                        map.insert(dest, dest_vec);

                        // part 2
                        let mut src_vec2 = map2.get(&src).unwrap().clone();
                        let mut dest_vec2 = map2.get(&dest).unwrap().clone();

                        let d: Vec<String> = src_vec2.drain(..num).collect();
                        dest_vec2.splice(..0, d);

                        map2.insert(src, src_vec2);
                        map2.insert(dest, dest_vec2);
                    },
                    _ => {
                        let mut offset = 0i32;
                        let mut position = 0i32;
                        for i in v.clone() {
                            if i == "" {
                                offset += 1;
                                if offset == 4 {
                                    position += 1;
                                    offset = 0;
                                }
                            } else {
                                position += 1;
                                let mut m = map.get(&position).unwrap().clone();
                                let mut m2 = map.get(&position).unwrap().clone();
                                m.push(i[1..i.len() - 1].to_string());
                                m2.push(i[1..i.len() - 1].to_string());
                                map.insert(position, m);
                                map2.insert(position, m2);
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 1..=9 {
        print!("{}", map.get(&i).unwrap()[0]);
    }
    print!("\n");

    for i in 1..=9 {
        print!("{}", map2.get(&i).unwrap()[0]);
    }
    print!("\n");
}