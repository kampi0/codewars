fn main() {
    println!("Hello, CodeWars!!!"); // for debuging to be deleted
    let s = String::from("This website is for losers LOL!Y");

    disemvowel(&s);
    
}

fn disemvowel(s: &str) -> String {

    let illegal_char_list = ['a','A','e','E','i','I','o','O','u','U'];
    let mut text_without_illegal_char = String::from("");
  
    for c in s.chars(){
        let mut char_up_loop_counter =0;
        let mut illegal_char_detector =0;
        while char_up_loop_counter != illegal_char_list.len(){
            
            if c == illegal_char_list[char_up_loop_counter]{
                illegal_char_detector +=1;
            }
            char_up_loop_counter += 1;
            // println!("{}", c)
        }

    if illegal_char_detector ==0{
        text_without_illegal_char.push(c)
        }
         println!("{}", text_without_illegal_char)
    }
    

    text_without_illegal_char
}
