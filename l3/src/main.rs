fn main() {
    let x=1234567;
    let a=2;
    let b=3;

    let size=count_raz(x);
    if size>0 && a<=size && b<=size {
        println!("{}",swap(x,a,b,size));
    }
    else {
        println!("n/a");
    }
}

fn swap (mut x: u64, a:u8, b:u8, size:u8) -> u64{
    let mut mas: [u8;size];

    for ind in 0..size {
        mas[size-1-ind]=x%10;
        x/=10;
    }

    let temp = mas[a];
    mas[a]=mas[b];
    mas[b]=temp;

    for ind in 0..size {
        if ind==0 {
            x=mas[0];
        }
        else {
            x*=10;
            x+=mas[ind];
        }
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
