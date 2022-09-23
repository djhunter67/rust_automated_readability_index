use std::fs;

fn main() {
    // let test_word: String = String::from("Hello, world, one more. Hello, world!, one more.");

    let test_word = fs::read_to_string("gettysburg_address.txt").expect("Text should be readable");

    let words = count_words(&test_word);
    let characters = count_characters(&test_word);
    let sentences = count_sentences(&test_word);

    println!("Number of words: {}", words);
    println!("Number of letters: {}", characters);
    println!("Number of sentences: {}", sentences);
    println!("ARI number: {}", ari(words, characters, sentences))
}

// count words function
fn count_words(text: &String) -> u64 {
    let word_count_bytes = text.as_bytes(); // make the string an array of bytes

    let mut counter: u64 = 0;

    for &i in word_count_bytes.iter() {
        if i == b' ' {
            counter += 1;
        }
    }

    return counter + 1;
}

// count characters function
fn count_characters(text: &String) -> u64 {
    let incoming_text = text.as_bytes();

    let mut aggregator: u64 = 0;

    for &i in incoming_text.iter() {
        aggregator += 1;

        if i == b' '
            || i == b';'
            || i == b':'
            || i == b'!'
            || i == b'?'
            || i == b'-'
            || i == b'"'
            || i == b','
            || i == b'\''
            || i == b'!'
        {
            aggregator -= 1;
        }
    }

    return aggregator;
}

// count sentences function
fn count_sentences(text: &String) -> u64 {
    let mut aggegrator: u64 = 0;

    for &i in text.as_bytes().iter() {
        if i == b'.' {
            aggegrator += 1
        }
    }

    return aggegrator;
}

// define the ARI function
fn ari(words: u64, characters: u64, sentences: u64) -> u8 {
    let constant_1: f32 = 4.71;
    let constant_2: f32 = 0.5;
    let constant_3: f32 = 21.43;

    let ari_func = constant_1 * (&characters / &words) as f32
        + &constant_2 * (&words / &sentences) as f32
        - constant_3;

    return ari_func as u8;
}

// define quantify ARI function
