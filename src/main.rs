use std::io;
use rand::Rng; // import Rng trait เข้ามาใน scope เพื่อให้ใช้ method สุ่มเลขได้
use std::cmp::Ordering; // import Ordering enum เพื่อใช้ผลลัพธ์จากการเปรียบเทียบ

fn main() {
    println!("Guess the number!");

    // สร้างตัวแปรเก็บเลขลับ โดยเรียกใช้ generator แล้วสุ่มเลขในช่วง 1 ถึง 100
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // แปลง String เป็น i32 โดยใช้ parse() และ trim() เพื่อเอา newline ออก
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    // ใช้ match เพื่อเปรียบเทียบค่าที่เดา (guess) กับเลขลับ (secret_number)
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),    // กรณีที่ค่าน้อยกว่า
        Ordering::Greater => println!("Too big!"),   // กรณีที่ค่ามากกว่า
        Ordering::Equal => println!("You win!"),     // กรณีที่ค่าเท่ากัน
    }
}