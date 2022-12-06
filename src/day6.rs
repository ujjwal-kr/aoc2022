use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file: File = File::open("./day6/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    ret_position(4, contents.clone());
    ret_position(14, contents);
}
fn ret_position(l: usize, contents: String) {
    let mut v: Vec<&str> = contents.split("").collect();
    v = v[1..v.len()-1].to_vec();
    let mut t: Vec<&str> = vec![];
    let mut i = 0;
    for _ in v.clone() {
        let c: &str = v.drain(..1).collect::<Vec<&str>>()[0];
        if !t.contains(&c) || t.len() == l {
            if t.len() == l {
                println!("{}", i);
                break;
            }
        } else { t = vec![] }
        t.push(c);
        i += 1;
    };
}