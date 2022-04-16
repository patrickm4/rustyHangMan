use std::io;

fn main() {
    // we need to choose a random word

    // then show a clue and the amount of letters needed

    // need to show already guessed letters and not allow user to guess the same letter again

    println!("Welcome to rusty ropes!");
    let mut guessed_letters = Vec::new();

    loop {
        let mut guess_letter = String::new();
        let mut guess_word = String::new();
        let mut guess_type = String::new();

        if guessed_letters.len() > 0 {
            // TODO convert vec into chars and concat into a string
            println!("Letters guessed: {:?}", guessed_letters);
        }
        println!("Guess a letter or word? (L/W)");

        println!("guess_type: {}", guess_type);

        io::stdin()
            .read_line(&mut guess_type)
            .expect("Failed to read line");

        if guess_type.trim() == "W" {
            println!("What's the word?");

            io::stdin()
                .read_line(&mut guess_word)
                .expect("Failed to read line");

            // need to reset guess but this errors out
            // guess_type = "";
        } else if guess_type.trim() == "L" {
            println!("Guess a letter");

            io::stdin()
                .read_line(&mut guess_letter)
                .expect("Failed to read line");

            guessed_letters.push(guess_letter);
        } else if guess_type.trim() == "exit" {
            break
        }
    }
}
