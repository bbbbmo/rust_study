use std::{cmp::Ordering, io}; // 입출력(io)에 관련된 기능들을 제공하는 라이브러리 불러오기
use rand::Rng; // Rng는 난수 생성기를 구현한 메서드들을 정의한 트레이트 (trait)

pub fn guessing_game() { 
    println!("Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("시크릿 번호: {secret_number}");

    loop {
        println!("추측되는 번호를 입력해주세요");
    
        let mut guess = String::new(); // 빈 문자열을 초기값으로 가진 가변 변수 생성
    
        io::stdin()
            .read_line(&mut guess)
            .expect("잘못된 입력입니다");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
          
        println!("입력한 추측 값: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;
            },
        }
    }
}
