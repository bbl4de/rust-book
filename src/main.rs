use std::io;

fn main() {
    //provide_index();
    //let num = plus_one(1);
    //println!("{num}");
    //loops();
    let x = scope();
    println!("{x}");
}   

fn provide_index() {

    let a = [1,2,3,4,5,6];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read the line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    if index >= a.len() {
        println!("Wrong number");
    }


    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    }


fn plus_one(x: i32) -> i64{

    let y: i64 = (x + 1).into();
    y
}
fn loops() {
    let mut i = 0;
    while i < 10 {
        println!("small");
        i += 1;
    }
    let a = 1..40;
    for el in a {
        println!("{el}");
    }
    let mut counter = 0;
    'testLoop: loop  {
        counter += 1;
        if counter == 1 {
            break 'testLoop;
        } 
    }
}
fn scope() -> i32 {
    let  x:i32 = 5;

    modify(x)

}
fn modify(mut x: i32) -> i32 {
    x += 123;
    x
}
fn reference 