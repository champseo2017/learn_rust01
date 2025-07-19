use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    // ใช้ loop เพื่อให้ user เดาได้เรื่อยๆ แบบไม่จำกัด
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // เปลี่ยนจาก .expect() มาใช้ match เพื่อจัดการ error ได้อย่างยืดหยุ่น
        // สังเกตว่าเราเปลี่ยน type เป็น u32 ตามตัวอย่าง
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,      // ถ้าแปลงเป็นตัวเลขสำเร็จ (Ok) ก็ให้ค่าตัวเลขนั้นออกมา
            Err(_) => continue,  // ถ้าแปลงไม่สำเร็จ (Err) ให้ข้ามไปเริ่ม loop รอบใหม่ด้วย continue
        };

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