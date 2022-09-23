use std::fs;

struct Age {
    age_1: String,
    age_2: String,
    age_3: String,
    age_4: String,
    age_5: String,
    age_6: String,
    age_7: String,
    age_8: String,
    age_9: String,
    age_10: String,
    age_11: String,
    age_12: String,
    age_13: String,
    age_14: String,
}

struct GradeLevel {
    grade_level_1: String,
    grade_level_2: String,
    grade_level_3: String,
    grade_level_4: String,
    grade_level_5: String,
    grade_level_6: String,
    grade_level_7: String,
    grade_level_8: String,
    grade_level_9: String,
    grade_level_10: String,
    grade_level_11: String,
    grade_level_12: String,
    grade_level_13: String,
    grade_level_14: String,
}

fn main() {
    // let test_word: String = String::from("Hello, world, one more. Hello, world!, one more.");

    let filename: &str = "gettysburg_address.txt";

    let test_word = fs::read_to_string(&filename).expect("Text should be readable");

    let words = count_words(&test_word);
    let characters = count_characters(&test_word);
    let sentences = count_sentences(&test_word);
    let ari = ari(words, characters, sentences);

    println!("Number of words: {}", words);
    println!("Number of letters: {}", characters);
    println!("Number of sentences: {}", sentences);
    println!("ARI number: {}", ari);

    println!("The ARI for {} is {}", &filename, &ari);
    println!("This corresponds to a(n) {} level of difficulty", ari_results(&ari).1);
    println!("That is suitable for an average person {} years old", ari_results(&ari).0)
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

    return ari_func.ceil() as u8;
}

// define ARI results
fn ari_results(ari_number: &u8) -> (String, String) {
    let ages: Age = Age {
        age_1: String::from("5-6"),
        age_2: String::from("6-7"),
        age_3: String::from("7-8"),
        age_4: String::from("8-9"),
        age_5: String::from("9-10"),
        age_6: String::from("10-11"),
        age_7: String::from("11-12"),
        age_8: String::from("12-13"),
        age_9: String::from("13-14"),
        age_10: String::from("14-15"),
        age_11: String::from("15-16"),
        age_12: String::from("16-17"),
        age_13: String::from("17-18"),
        age_14: String::from("18-19"),
    };

    let grade: GradeLevel = GradeLevel {
        grade_level_1: String::from("Kindergarten"),
        grade_level_2: String::from("1st Grade"),
        grade_level_3: String::from("2nd Grade"),
        grade_level_4: String::from("3rd Grade"),
        grade_level_5: String::from("4th Grade"),
        grade_level_6: String::from("5th Grade"),
        grade_level_7: String::from("6th Grade"),
        grade_level_8: String::from("7th Grade"),
        grade_level_9: String::from("8th Grade"),
        grade_level_10: String::from("9th Grade"),
        grade_level_11: String::from("10th Grade"),
        grade_level_12: String::from("11th Grade"),
        grade_level_13: String::from("12th Grade"),
        grade_level_14: String::from("College"),
    };

    match ari_number {
        1 => (ages.age_1, grade.grade_level_1),
        2 => (ages.age_2, grade.grade_level_2),
        3 => (ages.age_3, grade.grade_level_3),
        4 => (ages.age_4, grade.grade_level_4),
        5 => (ages.age_5, grade.grade_level_5),
        6 => (ages.age_6, grade.grade_level_6),
        7 => (ages.age_7, grade.grade_level_7),
        8 => (ages.age_8, grade.grade_level_8),
        9 => (ages.age_9, grade.grade_level_9),
        10 => (ages.age_10, grade.grade_level_10),
        11 => (ages.age_11, grade.grade_level_11),
        12 => (ages.age_12, grade.grade_level_12),
        13 => (ages.age_13, grade.grade_level_13),
        14 => (ages.age_14, grade.grade_level_14),
        &0_u8 | &15_u8..=u8::MAX => todo!(),
    }
}
