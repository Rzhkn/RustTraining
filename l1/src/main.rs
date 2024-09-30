fn main() {
    let mut x = 12345;
    let mut mas = [0;5];
    let size = mas.len();

    for i in 0..size {
        mas[size-1-i]=x%10;
        x/=10;
    }

    let temp = mas[1];
    mas[1]=mas[3];
    mas[3]=temp;

    for i in 0..size {
        if i==0 {
            x=mas[i];
        }
        else {
            x*=10;
            x+=mas[i];
        }
    }

    println!("{x}");
}
