use std::io; // 입출력(io)에 관련된 기능들을 제공하는 라이브러리 불러오기

fn main() { // main 함수 선언 - 프로그램에서 가장 먼저 시작되는 특별한 함수
    println!("Guess the number!"); // println!은 문자열을 화면에 출력하는 매크로(!가 붙으면 매크로임)

    println!("추측되는 번호를 입력해주세요");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Faild to read line");

    println!("입력한 추측 값: {guess}");
}
