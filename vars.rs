
pub fn run(){
    const ID: i16 = 3;
    println!("My ID is {}", ID);

    let name = "Dragon";
    let mut age = 33;
    println!("My name is {} and age is {}", name, age);

    age = 34;
    println!("My name is {} and age is {}", name, age);

    let (my_name, my_age) = ("Love", "Infinitum");
    println!("My name is {} and age is {}", my_name, my_age);
}