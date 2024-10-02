fn main() {
    let x: u64 = 32987237529;
    println!("{}",count_raz(x));
}

fn count_raz (mut x: u64) -> u8 {
    let mut res: u8 = 0;
    while x!=0 {
        x/=10;
        res+=1;
    }
    res
}
