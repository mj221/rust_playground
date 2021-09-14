// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3

//     println!("{}", s3);
// } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("hello"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//   let s = String::from("hello");  // s comes into scope

//   takes_ownership(s.clone());             // s's value moves into the function...
//                                   // ... and so is no longer valid here

//   let x = 5;                      // x comes into scope

//   makes_copy(x);                  // x would move into the function,
//                                   // but i32 is Copy, so it's okay to still
//                                   // use x afterward
//   println!("{}", s);
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
// // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//   println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//   println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// mod back_of_house {
//   pub struct Breakfast {
//       pub toast: String,
//       seasonal_fruit: String,
//   }
//   pub enum Appetizer {
//     Soup,
//     Salad,
// }
//   impl Breakfast {
//       pub fn summer(toast: &str) -> Breakfast {
//           Breakfast {
//               toast: String::from(toast),
//               seasonal_fruit: String::from("peaches"),
//           }
//       }
//   }
// }

// pub fn eat_at_restaurant() {
//   // Order a breakfast in the summer with Rye toast
//   let mut meal = back_of_house::Breakfast::summer("Rye");
//   // Change our mind about what bread we'd like
//   meal.toast = String::from("Wheat");
//   println!("I'd like {} toast please", meal.toast);
  

//   // The next line won't compile if we uncomment it; we're not allowed
//   // to see or modify the seasonal fruit that comes with the meal
//   // meal.seasonal_fruit = String::from("blueberries");
// }
// fn main(){
//   eat_at_restaurant();
// }

// mod front_of_house;

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// fn main(){
//   eat_at_restaurant();
// }


// fn main() {
//   let mut v = vec![100, 32, 57];
//   println!("{:?}", v.pop());
//   println!("{}", v[1])
// }

fn main() {
  let s1 = "hello";
  for c in s1.chars(){
    println!("{}", c);
  }
}



