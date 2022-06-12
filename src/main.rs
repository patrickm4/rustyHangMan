use std::io;

fn main() {
    // we need to choose a random word

    // then show a clue and the amount of letters needed

    // need to show already guessed letters and not allow user to guess the same letter again

    println!("Welcome to rusty ropes!");
    let _guessed_letters: Vec<String> = Vec::new();
    let mut str_guessed_letters = String::new();

    loop {
        let mut guess_letter = String::new();
        let mut guess_word = String::new();
        let mut guess_type = String::new();

        if str_guessed_letters.len() > 0 {
            println!("{}", str_guessed_letters);
        }

        println!("Guess a letter, word or stop playing? (L/W/S)");

        println!("guess_type: {}", guess_type);

        io::stdin()
            .read_line(&mut guess_type)
            .expect("Failed to read line");

        if guess_type.trim() == "W" {
            println!("What's the word?");

            io::stdin()
                .read_line(&mut guess_word)
                .expect("Failed to read line");

        } else if guess_type.trim() == "L" {
            println!("Guess a letter");

            io::stdin()
                .read_line(&mut guess_letter)
                .expect("Failed to read line");

            str_guessed_letters.push_str(&guess_letter.trim());
        } else if guess_type.trim().to_lowercase() == "s" {
            println!("Bye!");
            break
        }
    }
}
