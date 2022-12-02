fn count_bits(n: i64) -> u32 {
    let mut num = n;
    let mut count: u32 = 0;
    
    while num > 1 {
        if num % 2 == 1 {
            count += 1;
        } 
        num /= 2;
    }
    
    if n != 0 {
        count += 1
    }
    
    return count;
}
