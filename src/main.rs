use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    // ใช้ loop เพื่อให้ user เดาได้เรื่อยๆ แบบไม่จำกัด
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // แปลง input เป็นตัวเลข ถ้าใส่ค่าอื่นที่ไม่ใช่ตัวเลข โปรแกรมจะ crash
        let guess: i32 = guess.trim().parse().expect("Please type a number!");

        // เปรียบเทียบค่าที่เดากับเลขลับ
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // เพิ่ม break ที่นี่ เพื่อสั่งให้ออกจาก loop เมื่อทายถูก
            }
        }
    } // จบ loop แล้ววนกลับไปทำงานข้างบนใหม่
}