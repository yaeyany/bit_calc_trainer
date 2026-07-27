use std::io;
use rand::{self, random_range};

pub fn mode_1() {

    loop {
        let a = random_range(1..=u8::MAX);

        let a_bin = format!("{:b}", a);
        println!("Given binary: {}. Input your decimal answer. To quit print 0", a_bin);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed guess");
        let guess_trim: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please write a number");
                    continue;
                }
            };

        if guess_trim == a {
            println!("Good, it was {}", a);
        }   else if guess_trim == 0 {
            break;
        }   else {
            println!("Wrong, it was {}", a);
        } 
    }

}