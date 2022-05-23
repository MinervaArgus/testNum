use std::io;
fn main(){
    let tcases: i64 = get_num();
    for i in 0..tcases {
        let n: i64 = get_num();
        match n{
            i64::MIN..=1399 => println!("Division 4"),
            1400..=1599 => println!("Division 3"),
            1600..=1899 => println!("Division 2"),
            1900..=i64::MAX => println!("Division 1")
        }
    }
}

fn get_num () -> i64 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let n: i64 = num.trim().parse().unwrap();
    return n;
}


