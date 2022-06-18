use std::io;
use rand::Rng;

fn main() {
    // we need to choose a random word
    let cars = ["acura", "bmw", "alfa romeo", "aston martin", "honda", "toyota"];
    let random_num = rand::thread_rng().gen_range(0..6);
    let magic_word = cars[random_num];

    // TODO then show a clue and the amount of letters needed

    // TODO need to not allow user to guess the same letter again

    println!("Welcome to rusty ropes!");
    let _guessed_letters: Vec<String> = Vec::new();
    let mut str_guessed_letters = String::new();
    let mut person_to_hang = String::new();
    let mut hang_counter: u8 = 0;

    let magic_word_amount = magic_word.chars().count();

    let mut word_in_progress: Vec<String> = Vec::new();

    for _e in 0..magic_word_amount {
        word_in_progress.push("_".to_string())
    }

    // println!("{} {} {:?}", magic_word, magic_word_amount, word_in_progress);

    loop {
        let mut guess_letter = String::new();
        let mut guess_word = String::new();
        let mut guess_type = String::new();

        println!("Word to guess: {}", word_in_progress.join(" "));

        if str_guessed_letters.len() > 0 {
            println!("{}", str_guessed_letters);
        }

        if hang_counter > 0 {
            println!("{}", person_to_hang);
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

            if guess_word.trim().to_lowercase() == magic_word {
                println!("You win!");
                break
            } else {
                println!("Wrong! Try again");
            }
        } else if guess_type.trim() == "L" {
            println!("Guess a letter");


            io::stdin()
                .read_line(&mut guess_letter)
                .expect("Failed to read line");

            let guessed_letter = guess_letter.trim().to_lowercase();

            str_guessed_letters.push_str(&guessed_letter);

            if magic_word.contains(&guessed_letter) {
                println!("letter in word! {:?}", word_in_progress);
                // TODO fill word if letter is correct

                // find index letter is at
                let index = magic_word.find(&guessed_letter);
                //let index = magic_word.chars().position(|c| c == &guessed_letter).unwrap();

                // loop thru magic word to find all instancecs of the letters
                //for letter in magic_word.chars() {
                //    println!("{}", letter);
                //}

                let plusIndex: u8 = index + 1;

                // std::mem::replace(&mut word_in_progress[index], &guessed_letter);
                word_in_progress.replace_range(index..plusIndex, &guessed_letter);

                dbg!(index);
                // use index to place letter


            } else {
                match hang_counter {
                    0 => person_to_hang.push_str(" | "),
                    1 => person_to_hang.push_str("\n O͍ "),
                    2 => person_to_hang.push_str("\n/|\\"),
                    3 => {
                        person_to_hang.push_str("\n/ \\");
                        println!("{}", person_to_hang);
                        println!("You Lose!");
                        break
                    },
                    _ => println!("No Match"),
                }

                hang_counter = hang_counter + 1
            }

        } else if guess_type.trim().to_lowercase() == "s" {
            println!("Bye!");
            break
        }
    }
}
