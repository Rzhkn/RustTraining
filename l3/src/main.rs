fn main() {
    let x: u64 = 123456789123456789;
    let a: u64 = 1;
    let b: u64 = 18;

    let size=count_raz(x);
    if size>0 && a>0 && b>0 && a<=size.into() && b<=size.into() {
        println!("{}",swap(x,a,b,size as u64));
    }
    else {
        println!("n/a");
    }
}

fn swap (mut x: u64, a:u64, b:u64, size:u64) -> u64{
    let mut mas: Vec<u64> = Vec::new();

    while x!=0 {
        mas.push(x%10);
        x/=10;
    }

    let temp = mas[(size - a) as usize];
    mas[(size - a) as usize]=mas[(size - b) as usize];
    mas[(size - b) as usize]=temp;

    x=mas.pop().unwrap();
    while mas.len()>0 {
        x*=10;
        x+=mas.pop().unwrap();
    }

    x
}

fn count_raz (mut x: u64) -> u8 {
    let mut res: u8 = 0;
    while x!=0 {
        x/=10;
        res+=1;
    }
    res
}
