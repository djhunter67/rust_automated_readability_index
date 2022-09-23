// count words function
fn count_words(text: &String) -> u64 {
    
    let word_count_bytes = text.as_bytes();  // make the string an array of bytes

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

        if i == b' ' || i == b';' || i == b':' || i == b'!' || i == b'?' || i == b'-' || i == b'"' || i == b',' || i == b'\'' || i == b'!' {
            aggregator -= 1;
        }
        
    }

    return aggregator;
}

// count sentences function
fn count_sentences(text: &String) -> u32{

    let mut aggegrator: u32 = 0;

    for &i in text.as_bytes().iter() {

        if i == b'.' {
            aggegrator += 1
        }
    }

    return aggegrator;
}

// define the ARI function
fn ari(word: u64, characters: u64, sentences: u32) -> u8

// define quantify ARI function

fn main() {
    let test_word: String = String::from("Hello, world!, one more. Hello, world!, one more.");

    println!("Number of words: {}", count_words(&test_word));
    println!("Number of letters: {}", count_characters(&test_word));
    println!("Number of sentences: {}", count_sentences(&test_word));
}
