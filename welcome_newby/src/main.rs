/* A better way to use match (switch/case)
 *  get user input of String and Int
 *  Use the Int input to display a match statement
 *
 * */
#![allow(unused)]
use std::io::*;


fn main() {
    let mut name = String::new();
    let mut numb = String::new();

    println!("Welcome Rustling!!\nEnter your name: ");
    stdin().read_line(&mut name).expect("YOUR NAME!!");

    //Display the user name then accept a numerical value.
    println!("Welcome to Rust Land {}!!\nPlease pick a number between 1 - 4: ", name);
    stdin().read_line(&mut numb).expect("A NUMBER MORON!!");

    //Convert the String to an integer.
    let converted = numb.trim().parse::<u32>();

    //Match statement display
    match converted {

        Ok(1) => println!("One for the money!"),
        Ok(2) => println!("Two for the show!!"),
        Ok(3) => println!("Three to get ready!!"),
        Ok(4) => println!("Four...Lets GO!!!"),
        _ => println!("You FOOL, between 1 and 4 "),
    }



}
