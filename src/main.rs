// นำ io library เข้ามาใน scope เพื่อให้สามารถใช้งานฟังก์ชันด้าน input/output ได้
use std::io;

// function `main` คือจุดเริ่มต้นของโปรแกรม
fn main() {
    // print ข้อความต้อนรับและบอกให้ user ใส่ตัวเลขที่เดา
    println!("Guess the number!");
    println!("Please input your guess.");

    // สร้างตัวแปรแบบ mutable (แก้ไขค่าได้) ชื่อ guess เพื่อเก็บค่าที่ user พิมพ์
    // โดยกำหนดให้เป็น String เปล่าๆ ในตอนแรก
    let mut guess = String::new();

    // อ่านค่าที่ user พิมพ์จาก standard input (keyboard)
    io::stdin()
        .read_line(&mut guess) // อ่านค่าที่ user พิมพ์มาทั้งบรรทัดแล้วเก็บไว้ในตัวแปร guess
        .expect("Failed to read line"); // จัดการกับ error ที่อาจเกิดขึ้นหากอ่านค่าไม่สำเร็จ

    // print ค่าที่ user เดาออกมา โดยใช้ตัวแปร guess ที่รับค่ามาแล้ว
    println!("You guessed: {guess}");
}