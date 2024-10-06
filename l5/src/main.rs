fn main() {
    let x: i64 = 6324;
    let m = 100;
    
    if x>0 {
        println!("{}",count_wallets(count_bills(x),m));
    }
    else {
        println!("n/a");
    }
}

fn count_bills (mut x: i64) -> i32 {
    let mas: [i64;5] = [5000,1000,100,50,10];
    let mut count = 0;

    for num in mas {
        while x-num>=0 {
            count+=1;
            x-=num;
        }
    }

    count
}

fn count_wallets (mut bills: i32, m: i32) -> u32 {
    let mut wallets = 0;

    while bills-m>0 {
        wallets+=1;
        bills-=m;
    }

    wallets+1
}