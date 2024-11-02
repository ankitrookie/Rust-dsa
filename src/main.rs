mod bs_math;
mod gcd;
mod palindrome_num;

fn main() {
    /* Tokens are not standalone currencies; they are built on top of
    existing blockchains like Ethereum (ETH) or Solana (SOL). For example,
    if I create my own token called myCoin, it is considered a token, not
    a standalone currency. To make my token functional and transactional,
    it needs to be governed by smart contracts that define its rules and
    operations. */
    let x = 51015;
    // let val = bs_math::bsmath(x);
    // let val2 = rev_digit::revdigit(x);
    // let val3 = palindrome_num::palindromenum(x);
    let val4 = palindrome_num::palindromenum(x);
    // Below is count the characters
    println!("{}", val3);
    //    let mut count = 0;
    //   for _ in x.to_string().chars() {
    //      count += 1;
    // }
}

fn first_pattern(x: i32) {
    for _ in 0..x {
        for _ in 0..x - 1 {
            print!("*")
        }
        println!("*")
    }
    println!();
}

fn second_pattern(x: i8) {
    for i in 0..x {
        for _ in 0..i {
            print!("*")
        }
        println!("*")
    }
}

fn third_pattern(x: usize) {
    for i in 1..=x {
        for j in 1..=i {
            print!("{}", j)
        }
        println!()
    }
    println!();
}

fn fourth_pattern(x: usize) {
    for i in 1..=x {
        for _ in 1..=i {
            print!("{}", i)
        }
        println!()
    }
    println!();
}

fn fifth_pattern(x: usize) {
    for i in 1..=x {
        for _ in (i..=x).rev() {
            print!("*")
        }
        println!()
    }
    println!();
}

fn sixth_pattern(x: usize) {
    for i in (1..=x).rev() {
        for j in 1..=i {
            print!("{}", j)
        }
        println!()
    }
    println!();
}

fn seventh_pattern(x: usize) {
    for i in 0..x {
        for _ in 0..(x - i - 1) {
            print!(" ")
        }

        for _ in 0..(2 * i + 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
    println!();
}

fn eighth_pattern(x: usize) {
    for i in (0..x).rev() {
        for _ in 0..(x - i - 1) {
            print!(" ")
        }

        for _ in 0..(2 * i + 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
    println!();
}

fn ninth_pattern_1(x: usize) {
    for i in 0..x {
        for _ in 0..(x - i - 1) {
            print!(" ")
        }

        for _ in 0..(2 * i + 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
}

fn ninth_pattern_2(x: usize) {
    for i in (0..x).rev() {
        for _ in 0..(x - i - 1) {
            print!(" ")
        }

        for _ in 0..(2 * i + 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
}

fn tenth_pattern_1(x: usize) {
    for i in 0..x {
        for _ in 0..(x - i - 1) {
            print!("")
        }

        for _ in 0..(2 * i + 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
}

fn tenth_pattern_2(x: usize) {
    for i in (0..x).rev() {
        for _ in 0..(x - i - 1) {
            print!("")
        }

        for _ in 0..(2 * i - 1) {
            print!("*")
        }

        for _ in 0..(x - i - 1) {
            print!(" ")
        }
        println!()
    }
}

fn elvn_pattern(x: usize) {
    let mut start: usize;
    for i in 0..x {
        if i % 2 == 0 {
            start = 1
        } else {
            start = 0
        }

        for _ in 0..=i {
            print!("{}", start);
            start = 1 - start;
        }
        println!()
    }
    println!();
}

fn twelvth_pattern(x: usize) {
    let mut start: usize;
    for i in 0..x {
        start = if i % 2 == 0 { 1 } else { 0 };
        for _ in 0..=i {
            print!("{}", start);
            start = 1 - start;
        }
        println!()
    }
    println!();
}
