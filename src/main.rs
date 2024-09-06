use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

  loop {
      println!("1 to start game ");
      println!("4 to Exit in game");
      let mut input = String::new();

      io::stdin().read_line(&mut input).expect("error");
      let choise = input.trim().parse().expect("hello");
      match choise {
        1 => {
          guest();
        },
        _ => break,
          
      }
    
      }
  
}

fn guest(){
      
    println!("Guest A number from 1 - 20");
     println!("Enter A number");
     let mut guest = String::new();
     io::stdin().read_line(&mut guest).expect("error occurs");

     let secretnum = rand::thread_rng().gen_range(1..=10);
     let index = secretnum.to_string();
     
     match guest.cmp(&index) {
         
         Ordering::Less=>println!("Too Small {guest}the sec Number {secretnum}"),
         Ordering::Greater => println!("Is Bigger Guest Number is {guest} and secret number {secretnum}"),
         Ordering::Equal => println!("You Win "),
     }

}