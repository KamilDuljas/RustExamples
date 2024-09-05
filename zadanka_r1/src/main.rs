use std::io;

fn main() {
    println!("Hello, world!");
    zad1_1();
}

fn zad1_1() -> ()
{
    let mut a = String::new();

    println!("Podaj liczbe");
    io::stdin().read_line(&mut a)
    .expect("Something wrong during read line");

    let a: u8 = a.trim().parse().expect("Something wrong during parsing");
    let mut result: u32 = 0;
    for i in 3..a {
        if i % 3 == 0 || i % 5 == 0
        {
            result += i as u32;
        }
    }
    println!("Suma liczb podzielnych przez 3 lub 5 to {}",result);

}
