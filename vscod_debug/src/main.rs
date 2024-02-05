
fn main() {
    let  mut a = 5;
    let mut b = 7;

let result = add_numbers(a, b);
    println!("The result is: {}", result);

 a = 10;
b = 15;
 let result2 = add_numbers(a, b);
    println!("The result is: {}", result2);

}

fn add_numbers(x: i32, y: i32) -> i32 {
 
    let sum = x + y;


    sum
}
