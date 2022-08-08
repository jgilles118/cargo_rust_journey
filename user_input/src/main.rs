/*  To get input from the user and other stuff
 *
 * */
#![allow(unused)]
 use std::io::*;

 fn main() {
     //User input variable
     let mut user_in = String::new();
     
     println!("Enter you name: ");
     stdin().read_line(&mut user_in).expect("Use the alphabet only jerk!!...Mission failed");
 
     println!("Hello {}",user_in);

     println!("Enter a number between 1 - 5: "); 
     stdin().read_line(&mut user_in.flush()).expect("Failure to input a character jerk!!...Mission failed"); 
     
     println!("Your String selection: {}",us_2);
     //convert the string into an int
     //let numb_in = user_in as u16;

     //println!("Your int selection is: {}",numb_in);

     /*
      match numb_in{
        1 => println!("One for the money!"),
        2 => println!("Two for the show!"),
        3 => println!("Three to get ready!"),
        4 => println!("Four...Lets GO!"),
        5 => println!("Five just staying alive!"),
        _=> println!("Pick a number between 1 and 5 moron!"),

     }*/
        
 }
