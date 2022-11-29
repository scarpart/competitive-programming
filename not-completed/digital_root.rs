fn digital_root(n: i64) -> i64 {
    
    let mut mutable_n: i64 = n;
    
    while mutable_n >= 10 {
    
        let mut sum = 0;
        let mut i: usize = 0;
        let mut stringified: String = mutable_n.to_string();
        
        while stringified.len() > 1 {
            sum += stringified.chars().nth(i).unwrap() as i64;
            stringified = stringified.chars().skip(1).collect();
            println!("{}", stringified);
            mutable_n = sum;
            i += 1;
        }
    }
    
    return n;
    
}

fn main() {
    println!("{}", digital_root(235));
}
