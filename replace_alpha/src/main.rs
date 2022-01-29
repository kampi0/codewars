
fn main() {
    
    let text = String::from("The sunset sets at twelve o' clock.");
    println!("{}",alphabet_position(&text));

}

fn alphabet_position(text: &str) -> String 
    {
    let char_vec: Vec<char> = text.chars().collect();
    let alphabet_vec: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
// to DOauto lenght bese on char_vec
    let position_vec: Vec<&str> = vec!["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15","16","17","18","19","20","21","22","23","24","25","26"];  
    let mut result_vec = Vec::new();
    for c in char_vec 
        {
        // println!("{}",c);
        let mut main_loop_counter =0;
        while main_loop_counter != alphabet_vec.len()
            {
            if c.to_ascii_lowercase() == alphabet_vec[main_loop_counter]
                {
                result_vec.push(position_vec[main_loop_counter]);
                
                // println!("Positive match! {}", c);
                break
                }
            main_loop_counter +=1;
            }
            
        }
    let mut results = String::new();

    for x in result_vec 
        {
        
        results.push_str(x);
        results.push_str(" ");
        }
    results.pop();
    
    String::from(results)   
    }
