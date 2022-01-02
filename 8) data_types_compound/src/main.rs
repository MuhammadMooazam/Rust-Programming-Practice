fn main() {
    // Compound Data Types

    // Tuple
    let student: (&str, u32, f64, bool) = ("Mooazam", 21, 82.24, true);
    println!("Name is {}", student.0);
    println!("Age is {}", student.1);
    println!("Percentage is {}", student.2);
    println!("Passed is {}", student.3);
    println!("Now by Destructuring");

    let (a, b, c, d) = student;
    println!("Name is {}", a);
    println!("Age is {}", b);
    println!("Percentage is {}", c);
    println!("Passed is {}", d);

    // Array
    let lottery_number:[u32;4] = [10 , 20 , 30 , 40];
    println!("{:#?}" , lottery_number);

    let rupee = [5;3];  //[value,length]
    println!("{:#?}" , rupee);
    
    let days = ["Monday" , "Tuesday" , "Wednesday"];
    println!("First day is {:#?}" , days[0]);
}
