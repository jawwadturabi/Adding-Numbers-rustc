use std::io;
use std::cmp;
fn main() {
    println!("\n\t\t\tAddition of two Numbers : \n\n");
    
   
   loop
{  
        println!("Enter 1st Number = " );
        let mut first=String::new();
        io::stdin().read_line(&mut first)
            .expect("Failed to read line");
        let first: u32=match first.trim().parse()
        {
            Ok(num)=>num,
            Err(_)=>continue,
        };
      
     
     
      
        println!("Enter 2nd Number = " );
        let mut second=String::new();
        io::stdin().read_line(&mut second)
            .expect("Failed to read line");
        let second: u32=match second.trim().parse() 
        {
            Ok(num)=>num,
            Err(_)=>continue,
        };
       
    

    
    let sum = first+second;
    println!("\n\t\t\tThe Addition of two numbers is = {} \n",sum);

 }
}
