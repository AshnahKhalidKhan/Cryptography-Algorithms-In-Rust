/*
    The code contains three 'constants' that will be used throughout its execution:
    - Σ: an xhar array of numbers corresponding to an alphabet
    - Γ: a HashMap of alphabets in char mapped to a usize number
    - dictionary: a vector of Strings populated from reading the 'words.txt'


    Upon entering the keyword 'encrypt', this is how our code functions:
    - In the command line arguments, we expect a text input and a 'shift' value
    - parse() converts the text input into a String value
    - encrypt():
        • Takes this String and shift value as inputs
        • Creates a mutable empty String variable
        • Iterates over every character in the input String
        • If the character is alphabetic,
        it finds the corresponding new character value it would have after shifting it by the shift value,
            and then appends this character into the empty String variable created.
        • If the character was not alphabetic, it appends the character as it is into the empty String variable created as well
        • In the end, it outputs the String variable with the new appended shifted characters.


    Upon entering the keyword 'decrypt', this is how our code functions:
    - In the command line arguments, we expect a text input and a 'key' value
    - parse() converts the text input into a String value
    - decrypt():
        • Takes this String and key value as inputs
        • Converts the key into a 'key mod 26' value to account for values larger than 26
        • Note now that if the key value for an input text is x, that means each character in the original text was shifted x steps forward.
          In order to get the original character, we need to go x steps backward.
          Since the code only accepts usize values though, we can't go backwards.
          Also, since you can think of mod as a kind of circular queue that goes from 0 to 25 and then back to 0 again,
          to go x steps backward, we can instead just go '26 - x' steps forward.
        To shift characters any number of steps forward, we already have the encrypt() function.
            Hence, we call the encrypt() function with a shift value of '26 - key' value.
        • In the end, it outputs the String value of the decrypted text.


    Upon entering the keyword 'break', this is how our code functions:
    - Please first note that Rust was not allowing us to name a fuction called 'break()', so we stuck to calling it 'breaking()'
        (I would have liked to call it crack() but that just sounds wrong so...)
    - Secondly, it was for this part that it was necessary that we created a dictionary which we have done below, but I still don't understand how to make it a constant.
    - parseDictionary():
        • Takes a String value for filename as input
        • Creates a mutable String vector variable
        • Opens the file with the given filename using the std::fs::File library
        • Reads the file line by line using the std::io::BufReader and the .lines() method
        • Unwraps each line, which represents a single word in the given text file, so it can be treated as a String value
        • Uses to_ascii_uppercase() to make each character uppercase because later this will be used with the decrypt() function which compares only uppercase String values
        • Pushes each unwrapped, uppercased word into the String vector
        • In the end, it outputs a String vector of each word in the dictionary in an uppercased format.
    - The String vector output from the parseDictionary() function is used in the isWordInDictionary() for comparing real words generated from shift values.
    - In the command line arguments, we expect only a text input
     parse() converts the text input into a String value
    - breaking():
        • Takes this String value and the String vector dictionary we created as input
        • Creates a usize variable for keeping track of the shift value that returns the most real words
        • Creates another usize variable to count the number of most real words counted by any shift value yet
        • Note that ince we can think of shifting as a circular queue going from 0 to 25,
        we are sure that any key to decrypt the given text will be a multiple of any value from 0 to 25.
        • Iterates over shift values from 0 to 25
        • Decrypts the text according to each shift value
        • Creates a usize variable for counting real words in this decrypted text for the current shift value
        • Converts the decrypted String value into a String vector of words separated by whitespaces;
        uses an elaborate combination of functions including 'split_whitespace().into_iter().map(|s| s.to_string()).collect()'
        • Checks if each word belongs in the dictionary using the isWordInDictionary() function:
        - isWordInDictionary():
            • Takes String value of word to be found and dictionary to traverse as input
            • Uses binary search to find given String value in given Vector String dictionary
            • Returns true if found, and false if not found
        • If isWordInDictionary() returns true, increments the current shift value's real words count
        • After all words for this shift value have been checked,
        if the current shift value's real words count is greater than the maximum real words count for any shift value yet,
            set the maximum real words count to the current shift value's real words count,
            and the shift value that returned the maximum real words count yet to the current shift value.
        • In the end, output the shift value that returned the maximum real words count.


    Flaws remaining:
    - A lot of cloning going on to pass value
    - 'Σ', 'Γ' and 'dictionary' are still not constants yet
    - Making all characters uppercase is giving a _feeling_ that there might be something fishy going on with that, but I can't tell what cause the _feeling_ isn't strong enough yet :(
*/

use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main()
{
    pub const Σ: [char; 26] =
    [
      'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
      'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];
    
    let Γ: HashMap<char, usize> = HashMap::from
    ([
        ('A', 0), ('B', 1), ('C', 2), ('D', 3), ('E', 4), ('F', 5), 
        ('G', 6), ('H', 7), ('I', 8), ('J', 9), ('K', 10), ('L', 11), 
        ('M', 12), ('N', 13), ('O', 14), ('P', 15), ('Q', 16), ('R', 17),
        ('S', 18), ('T', 19), ('U', 20), ('V', 21), ('W', 22), ('X', 23),
        ('Y', 24), ('Z', 25) 
    ]);
    
    let dictionary: Vec<String> = parseDictionary("lesswords.txt".to_string());
    let input = parse();
    let mut encryption = encrypt(7, input.clone(), Σ.clone(), &Γ);
    let mut decryption = decrypt(7, encryption.clone(), Σ.clone(), &Γ);
    let mut breakkk = breaking(encryption.clone(), dictionary.clone(), Σ.clone(), &Γ);
    println!("Input: {}", input);
    println!("Encryption with shift 7: {}", encryption);
    println!("Decryption with shift 7: {}", decryption);
    println!("Breaking: {}", breakkk);
    
    // A few edge cases
    encryption = encrypt(42, input.clone(), Σ.clone(), &Γ);
    decryption = decrypt(42, encryption.clone(), Σ.clone(), &Γ);
    breakkk = breaking(encryption.clone(), dictionary.clone(), Σ.clone(), &Γ);
    println!("\nInput: {}", input);
    println!("Encryption with shift 42: {}", encryption);
    println!("Decryption with shift 42: {}", decryption);
    println!("Breaking: {}", breakkk);
}

fn parse() -> String
{
    let mut input_text = String::new();
    while let Some(Ok(line)) = io::stdin().lines().next()
    {
        for σ in line.chars()
        {
            if (σ.is_ascii_alphabetic())
            {
                input_text.push(σ.to_ascii_uppercase());
            }
            else
            {
                input_text.push(σ);
            }
        }
    }
    input_text
}

fn parseDictionary(filename: String) -> Vec<String>
{
    let mut dictionary: Vec<String> = Vec::new();
    let file = File::open(filename).expect("Error in reading file");
        let mut bufferReader = BufReader::new(file);
        for l in bufferReader.lines()
        {
            dictionary.push(l.unwrap().to_ascii_uppercase());
        }
    dictionary
}

fn isWordInDictionary(word: String, dictionary: Vec<String>) -> bool
{
    let mut first: usize = 0;
    let mut last: usize = dictionary.len() - 1;
    while (first <= last)
    {
        let mid = first + ((last - first) / 2);
        if (dictionary[mid] == word)
        {
            return true;
        }
        else if (word < dictionary[mid])
        {
            last = mid - 1;
        }
        else if (word >= dictionary[mid])
        {
            first = mid + 1;
        }
    }
    false
}

fn encrypt(shift: usize, words: String, Σ: [char; 26], Γ: &HashMap<char, usize>) -> String
{
    let mut OutputString: String = String::new();
    for character in words.chars()
    {
        if character.is_ascii_alphabetic() == true
        {
            let NumberOfCurrentAlphabet: usize = Γ[&character];
            let AlphabetOfNewNumber: char = Σ[(NumberOfCurrentAlphabet + shift) % 26];
            OutputString.push(AlphabetOfNewNumber);
        }
        else
        {
            OutputString.push(character);
        }
    }
    OutputString
}

fn decrypt(key: usize, words: String, Σ: [char; 26], Γ: &HashMap<char, usize>) -> String
{
    let key = key % 26;
    encrypt((26 - key), words.clone(), Σ.clone(), &Γ)
}

fn breaking(words: String, dictionary: Vec<String>, Σ: [char; 26], Γ: &HashMap<char, usize>) -> usize
{
    let mut maxCount: usize = 0;
    let mut shiftWithMaxCount: usize = 0;
    for shift in 0..=25
    {
        let shiftedWords: String = decrypt(shift, words.clone(), Σ.clone(), &Γ);
        let shiftedWordsAsVector: Vec<String> = shiftedWords.split_whitespace().into_iter().map(|s| s.to_string()).collect();
        let mut currentCount: usize = 0;
        for word in shiftedWordsAsVector
        {
            if isWordInDictionary(word, dictionary.clone()) == true
            {
                currentCount = currentCount + 1;
            }
        }
        if (currentCount > maxCount)
        {
            maxCount = currentCount;
            shiftWithMaxCount = shift;
        }
    }
    shiftWithMaxCount
}