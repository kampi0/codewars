fn main() 
{
    println!("Hello, world!");
    let input: Vec<(i32, i32)> =  vec![(18, 20), (45, 2), (55, -2), (37, 6), (55, 7), (78, 9)];
    println!("{:?}",open_or_senior(input));
}
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> 
    {
    let mut output: Vec<String>= Vec::new();
    for x in data
        {
            if x.0 >= 55 && x.1 > 7
                {
                output.push(String::from("Senior"));
                }
            else
                {
                output.push(String::from("Open"));
                }      
        }
    output
    
    }