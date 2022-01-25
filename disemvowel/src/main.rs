fn main() {
    println!("Hello, CodeWars!!!"); // for debuging to be deleted
    let s = String::from("This website is for losers LOL!");
    disemvowel(&s);
    
}

fn disemvowel(text_for_verification: &str) -> String {
    // let mut disemvowel_storage = vec![];
    let illegal_char_list = vec!['a','e','i','o','u','y'];
    let text_len: usize = text_for_verification.chars().count();
    let mut chars_in_text_counter: usize = 1;

    'characters_up: while chars_in_text_counter != text_len + 1 {
        let actual_char = text_for_verification.chars().take(chars_in_text_counter).last().unwrap(); //assigning characters from verified text
        println!("The letter number {} is: {} ", chars_in_text_counter, actual_char); // only for debuging
        let mut illegal_char_loop_counter: usize = 0;
        let mut illegal_char_found: usize = 0;

        'find_illegal: while illegal_char_loop_counter != illegal_char_list.len(){
            if actual_char == illegal_char_list[illegal_char_loop_counter]{
                break 'find_illegal;
            }
                else{
                    illegal_char_loop_counter += 1
                }
            


        }


        chars_in_text_counter += 1 
    }


    String::from("Returned String")
}


// fn disemvowel(s: &str) -> String {
  
//     let mut disemvowel_storage = vec![];
//     let disemvowel_list = vec!['a','e','i','o','u','y'];
//     let len = s.chars().count();
//     let mut number = 1;
    
    
//     println!("lenght is {}", len); // for debuging to be deleted
//     println!("1st character is {}", s.chars().take(1).last().unwrap()); // for debuging to be deleted

//     while number != len + 1 {   // len + 1 to cover all characters in Stirng
//         let actual_sign = s.chars().take(number).last().unwrap();
//         let mut number2 = 0;
//         let mut number3 = 0;
//         while number2 != disemvowel_list.len(){
//             if actual_sign == disemvowel_list[number2]{
//                 number3+=1
//             }
//             number2 +=1;
//                 if number3 == 0 {


//                     disemvowel_storage.push(actual_sign);
//                     println!("checking char number{}, {} and we have comare with {}",number, actual_sign, disemvowel_list[number2]  );
//                 }
            
//         }

//         // if s.chars().take(number).last().unwrap() != disemvowel_list {
//         //     disemvowel_storage.push(s.chars().take(number).last().unwrap()); 
//         // }
//         let index: &char =  &disemvowel_storage[disemvowel_storage.len()-1]; // for debuging to be deleted
//         println!(
//             "The letter number {} is: {}",
//             number,
//             index
//         );

        
        
//         number += 1
//     }
//     String::from("OK")
// }
