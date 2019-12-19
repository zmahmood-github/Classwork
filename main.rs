#[derive(Debug)]
struct Student {
    // key data type
    name: String,
    sub1: i32,
    sub2: i32, // comma at end is optional
}

fn main() {
    // let order = 500;
    // printer(order);// function calling

    // println!("Your order of {} books is ready", printer_with_reutrn_type()); // function calling

    //ownership
    // let name1 = "PIAIC"; // string Literal
    // println!("String LIteral name 1 : {}", name1);

    // let name = String::from("PIAIC"); // String Type

    // let receiver = dessert();
    // println!("{}", receiver);

    // let mut inputdata = String::new();
    // let int_data: u32 = 65;

    // match int_data {
    //     1 => println!("ONe"),
    //     2 => println!("Two"),
    //     3 => println!("Three"),
    //     _ => println!("Not in list"),

    let name = String::from("Farjal");
    let sub1 = 88;
    let sub2 = 77;
    let stdOne: Student = one(name, sub1, sub2);

    println!("returned studen = {:#?}", stdOne);

    let stud_tuple = (String::from("Zaheer"), 91, 81);

    //  two(stud_tuple);

    // println!("tuple = {:?}", stud_tuple);

    let mut student1 = Student {
        name: String::from("Noman AIC"),
        sub1: 95,
        sub2: 85, //comma at end is optional
    };

    let mut student2 = Student {
        name: String::from("Ahmed"),
        sub1: 80,
        sub2: 90,
    };

    print_student_name(student2.name);

    // println!("struct name: {}",student1.name);
    let sumOfSub1AndSub2 = three(student1);
    println!("Sum: {}", sumOfSub1AndSub2);

    // if int_data == 1 {
    //     println!("one");
    // } else if int_data == 3 {
    //     println!("Three");
    // } else {
    //     println!("Not in list");
    // }
}

fn print_student_name(name: String) {
    println!("Student Name is: {}", name);
}

fn one(name: String, sub1: i32, sub2: i32) -> Student {
    println!("name = {} , sub1 = {} , sub2 = {}", name, sub1, sub2);

    let student3 = Student {
        name: name,
        sub1: sub1,
        sub2: sub2,
    };

    // println!("student from struct : {:#?}", student3);
    student3 // return subject
}

fn two(stdTuple: (String, i32, i32)) {
    println!("tuple = {:?}", stdTuple);
}

fn three(stud: Student) -> i32 {
    // println!("struct_Student {:#?} ", stud);
    stud.sub1 + stud.sub2
}

fn dessert() -> String {
    let d1 = String::from("Multan Lava");
    d1
}

fn printer(copy: u32) // function signature
{
    println!("Book Printing");
    println!("Composing");
    println!("Draft Print");
    println!("ProofRead");
    println!("Editing/Updating");
    println!("Print {} copies", copy);
}

fn printer_with_reutrn_type() -> u32 // function signature
{
    println!("Book Printing");
    println!("Composing");
    println!("Draft Print");
    println!("ProofRead");
    println!("Editing/Updating");
    println!("Print 500 copies");
    500 //returning
}
