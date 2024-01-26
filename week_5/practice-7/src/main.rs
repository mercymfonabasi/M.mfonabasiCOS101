fn main() {

    let  count = 0;

    for num in 1..21{
        if num > 10{
            println!("{:?}",num);
            continue;
        }
    }
    println!("the count of values greater than 10 (between 1 and 20) is :{}",count);
    //outputs 10
}
