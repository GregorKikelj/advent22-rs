use std::vec;
fn main() {
    let input = include_str!("../../inputs/day1.txt").trim();
    let blocks: Vec<&str> = input.split("\n\n").collect();
    
    let mut sums: Vec<i32> = vec![];
    for block in blocks {
        let mut sum = 0;
        for line in block.lines(){
            let num: i32 = line.parse().unwrap();
            sum += num;
        }
        sums.push(sum);
    }
    sums.sort();
    println!("Part 1: {}", sums[sums.len()-1]);

    let mut sums2 = 0;
    for i in 0..3 {
      sums2+=sums[sums.len()-1-i];
    }
    println!("Part 2: {}", sums2);
}
