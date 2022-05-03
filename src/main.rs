use std::io;

fn main() {
    // we need to choose a random word

    // then show a clue and the amount of letters needed

    // need to show already guessed letters and not allow user to guess the same letter again

    println!("Welcome to rusty ropes!");
    // let mut guessed_letters: Vec<String> = Vec::new();
    let _guessed_letters: Vec<String> = Vec::new();
    let mut str_guessed_letters = String::new();

    loop {
        let mut guess_letter = String::new();
        let mut guess_word = String::new();
        let mut guess_type = String::new();

        // if guessed_letters.len() > 0 {
        //     // TODO convert vec into chars and concat into a string
        //     for i in &guessed_letters {
        //         // print!("{}", i);
        //         str_guessed_letters.push(i)
        //     }
        //
        //     // println!("Letters guessed: {:?}", guessed_letters);
        // }

        if str_guessed_letters.len() > 0 {
            println!("{}", str_guessed_letters);
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

        } else if guess_type.trim() == "L" {
            println!("Guess a letter");

            io::stdin()
                .read_line(&mut guess_letter)
                .expect("Failed to read line");

            // guessed_letters.push(guess_letter);
            // guess_letter.push(',');
            str_guessed_letters.push_str(&guess_letter.trim());
            // str_guessed_letters.push(guess_letter);
            // str_guessed_letters.push('y');
        } else if guess_type.trim() == "exit" {
            break
        }
    }
}
