fn main(){
    let name="Aisha Lawal";
    let uni:&str="Pan-atlantic University";
    let addr:&str="Km 52 Lekki-epe expressway,Ibeju-lrkki, Lagos";
    print!("Name:{}",name );
    println!("University: {}, \nAdress: {}",uni,addr);



    let department:&'static str ="Computer Science";   
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool:{}",department,school);
}
