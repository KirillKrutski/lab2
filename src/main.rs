use std::io;

fn is_prime(n: u32) -> bool{
    if n < 2 { 
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || {n % 3 == 0} {
        return false;
    }
    for i in (5..).step_by(6).take_while(|&i| i * i <= n){
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Enter the positive number: ");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed ti read line");

    let x: u32 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("error"),
    };
    
    if x < 2 {
        return;
    }

    let mut do_new_line = false;
    for i in 2..x{
        if i % 10 == 0{
            do_new_line = true;
        }
        if is_prime(i) == true {
            if do_new_line {
                println!();
                do_new_line = false;
            }
            print!("{} ", {i});
        }
       
    }

}
