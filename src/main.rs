use std::io;
use rand::Rng; // import Rng trait เข้ามาใน scope เพื่อให้ใช้ method สุ่มเลขได้

fn main() {
    println!("Guess the number!");

    // สร้างตัวแปรเก็บเลขลับ โดยเรียกใช้ generator แล้วสุ่มเลขในช่วง 1 ถึง 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}