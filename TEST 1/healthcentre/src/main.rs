use std::io

fn main(){
    let mut input1=string::new();
    let mut input2=string::new();
    let mut input3=string::new();
    let mut input4=string::new();
    let mut input5=string::new();
    let mut input6=string::new();
    let mut input7=string::new();
    let mut input8=string::new();
    
    println!("Enter your name: ");
    io:stdin().read_line(&mut input1).expect("Not a valid string")

    println!("Enter your date of birth");
     io:stdin().read_line(&mut input2).expect("Not a valid string")

      println!("Enter your email adress: ");
    io:stdin().read_line(&mut input3).expect("Not a valid string")

     println!("Enter your phone number: ");
    io:stdin().read_line(&mut input4).expect("Not a valid string")

     println!("Enter your number of siblings: ");
    io:stdin().read_line(&mut input5).expect("Not a valid string")

     println!("Enter your numer:of children ");
    io:stdin().read_line(&mut input6).expect("Not a valid string")

     println!("Enter your medical diagnosis: ");
    io:stdin().read_line(&mut input7).expect("Not a valid string")

     println!("Enter your village of rescidence: ");
    io:stdin().read_line(&mut input8).expect("Not a valid string")
}
