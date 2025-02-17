fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        return -1;
    } else if guess > secret {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let secret_number = 7;

    let mut guess = 10;

    let mut counter = 0;

    while guess != secret_number {
        if check_guess(guess, secret_number) == -1 {
            println!("Too low!");
            guess += 1;
        } else if check_guess(guess, secret_number) == 1 {
            println!("Too high!");
            guess -= 1;
        }

        counter += 1;
    }

    println!("You win! It took you {counter} tries.");
}
