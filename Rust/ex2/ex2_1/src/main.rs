use std::collections::HashMap;
fn main() {

    let mut int_list = vec![1,1,5,6,5,5];
    int_list.sort();

    println!("{:?}", int_list);

    let med = median(&int_list);

    println!("{:?}", med);

    let mode_val = mode(&int_list);

    println!("{:?}", mode_val);

}
fn median(int_list: &Vec<i32>) -> i32 {
    let mut sorted_list = int_list.clone();
    sorted_list.sort();

    let len = sorted_list.len();
    if len % 2 == 0 {
        let v1 = sorted_list[len / 2];
        let v2 = sorted_list[len / 2 - 1];
        (v1 + v2) / 2
    } else {
        sorted_list[len / 2]
    }
}
fn mode(int_list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &i in int_list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let (mode, _count) = map.iter().max_by_key(|&(_, count)| count).unwrap();
    *mode
}