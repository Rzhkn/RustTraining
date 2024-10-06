fn main() {
    let mut x: i64 = 10000000000;
    
    if x>0 {
        let mas: [i64;5] = [5000,1000,100,50,10];
        let mut count = 0;

        for num in mas {
            while x-num>=0 {
                count+=1;
                x-=num;
            }
        }
        println!("{}",count);
    }
    else {
        println!("n/a");
    }
}
