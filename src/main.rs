//simple calculator app in Rust with a console interface
//use borrow checking & variables/strings
//have plus, minus, substraction and division

use std::io;

fn calculate(arr: &mut Vec<String>){
    let vec: Vec<i32> = Vec::new();
    //println!("Length: {}", arr.len());
    //is odd/has operator
    let length = arr.len();
    if length==1{
        println!("You get back {}!",arr[0])
    }
    else if length % 2 == 1 {
        let mut counter = 1;
        let mut ans: i32=0;
        let mut _sign: &str="";
        let _op = ['+','-','*','/'];
        for x in 0..length{
            //println!("length is: {}", x);
            if counter%2==1{
                //if odd and not one, assumes there are operators
                //println!("{}", arr[x]);
                let num: i32 = arr[x].parse().unwrap();
                if ans == 0 {
                ans = num;
                }
                else {
                    match _sign {
                        "+" => { ans = ans + num },
                        "-" => { ans = ans - num },
                        "*" => { ans = ans * num },
                        "/" => { ans = ans / num},
                        _ => println! ("error"),
                    }
                }
                counter = counter+1;
            }
            else {
                //store operator in variable
                counter = counter+1;
                _sign = &arr[x];
                //println!("{}", _sign);
            }
                }
                println!("Final results are: {}", ans);
            }
            
        }
        
fn main() {
    println!("Hello there! Welcome to a simple calculator that returns whole numbers");
    let mut num_numbers = String::new();
    let stdin = io::stdin();
    println!("How many inputs will you be entering?");
    //get number of calculations
    stdin.read_line(&mut num_numbers);
    //take store input into array
    let num_number:usize = num_numbers.trim().parse().unwrap();
    let mut vector_numbers = vec![String::new();num_number];
    println!("After entering each input, please press Enter for the next input");
    for x in 0..num_number{
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("Please input numbers or operator");
        vector_numbers[x] = user_input.trim().into(); 
    }
    //println!("{:?}", vector_numbers);
    calculate(&mut vector_numbers);
    // let stdin = io::stdin();
    // println!("What is the first number?");
    // stdin.read_line(&mut user_input);
    // println!("What would you like to do with it?");
    // stdin.read_line(&mut user_input);
    // println!("What is the second number?");
    // stdin.read_line(&mut user_input);
    // println!("{}", user_input);


}
