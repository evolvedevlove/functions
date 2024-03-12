//use std::mem;


/*
Here’s a small programming problem: 
    write a function that takes
     a string of words separated by spaces
     and returns the first word it finds in that string. 
    If the function doesn’t find a space in the string,
     the whole string must be one word,
     so the entire string should be returned.
*/
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn square(num: i32) -> i32{
    num*num
}

fn draw_line(chr: String, nbr: usize) -> String {
    chr.repeat(nbr)
}

fn remove_letter(the_letter: char, the_string: &String) -> String {
    // a mutable string that will grow as we find the correct letters 
    let mut return_string = String::new();
    // arrays in rust have a fixed length
    // the return string must be made after we have gone through the loop
    // wait then why is there a .push_str and .push ??
    // so that means maybe i just dont understand yet
    // let mut v = Vec::new();
    // turns out in chapter8 of the rust book i will learn about vector collectors
    // so lets do this properly using a vector collector first
    let chars: Vec<char> = the_string.chars().collect();
    for letter in chars {
        if letter == the_letter {
            println!("found occurence of the no no letter");
            return_string.push(letter);
        } else {
            println!("{}", letter);
        }
    }
    // now would like to say all the letters in this vector are now the string
    return_string
}

fn analyze_slice(slice: &[i32]){
    println!("the first element of the slice is {}", slice[0]);
    println!("the slice has {} elements ", slice.len());
}

fn dcrust_csar(crusty: String, shifter: i32)-> String {
    let the_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut uncrusted = "";
    
    for letter in the_alphabet.as_bytes() {
        //let mut shifted_letter = &letter; 
        if the_alphabet.contains(letter) {
            let index = the_alphabet.chars().position(|r| r == letter).unwrap();
            // now handle if the handle rolls over to the next 
            // index - the_alphabet.length() 
            let shifted_letter = the_alphabet[index as i32 + shifter];
            println!("{shifted_letter}");
            uncrusted = &uncrusted + &shifted_letter;
        }

    }
    return uncrusted.to_string();
}

 //fn sum_a_list()

fn main() {
    let number =  25; 
    let _squared_number = square(number);
    println!("this is the number squared {} ",_squared_number);
    // write a program to comnpute the sum of numbers 
    // from 1 squared to the number enters (for now is hardcoded)
    let mut sum_of_squares = 0;
    for i in 1..number{
        if i <= number {
            sum_of_squares = sum_of_squares + square(i);
        } else {
            println!("we are done summing");
        }
    }
    
    // 1
    println!("the sum of squares is {}", sum_of_squares);

    // // 2
    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // stdin.read_line(&mut buffer)?;
    // Ok(())
    println!("{}", draw_line("big patty".to_string(), 6));

    // 3
    // write a program that omits a character from a string
    // when given a string and a character 
    let input_string = "patrick is a cool guy";
    let no_no_letter = 'a';
    let output_string = remove_letter(no_no_letter, &input_string.to_string());
    println!("{}", output_string);

    // 4
    // write a function that returns the average calculation of a list of numbers 
    // call the function twice - once for this list integer_numbers and one for a list with 

    // 5 - write function to return index of first space in program 
    let s = String::from("dad is a programmer");
    let the_first_word = first_word(&s);
    println!("the first word is {}", the_first_word);

    // 6 analyze a slice
    let analyze_me = [7, 69, 420, 17];
    analyze_slice(&analyze_me);
    analyze_slice(&analyze_me[1..]);
    
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]); 

    for i in 0..analyze_me.len() {
        match analyze_me.get(i){
            Some(a_value) => println!("{}: {}", i, a_value),
            None => println!("nothing to print!"),
        }
    }

    // 7. make a decrypt cypher function / it just shifts each letter down the alphabet
    let to_decrypt = String::from("vw");
    let shift = -3;
    let decrypted = dcrust_csar(to_decrypt, shift);
    println!("{}an is not what he seems", decrypted);
}
