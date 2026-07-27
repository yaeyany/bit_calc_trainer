use std::io;
use rand::{self, random_range};

pub fn mode_2() {
    loop {
        let a = random_range(1..=u8::MAX);

        let a_bin = format!("{:b}", a);
        println!("Given decimal: {}. Input your binary answer. To quit print 0", a);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed guess");
        let guess_trim= guess.trim().to_lowercase();

        if guess_trim == a_bin {
            println!("Good, it was {}", a_bin);
        }   else if guess_trim == "0" {
            break;
        }   else {
            println!("Wrong, it was {}", a_bin);
        } 
    }
}