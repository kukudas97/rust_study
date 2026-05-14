use std::cmp::Ordering; //비교 연산자 라이브러리에서 Ordering 열거형을 가져옴
use std::io; //java import개념(prelude라고 하며 표준라이브러리 문서에서 확인가능)

use rand::Rng; //rand 라이브러리에서 Rng 트레이트를 가져옴

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //1부터 100까지의 숫자 중에서 랜덤한 숫자를 생성하여 secret_number 변수에 저장

    println!("The secret number is: {secret_number}"); //생성된 랜덤 숫자를 출력(테스트용, 실제 게임에서는 이 부분을 제거해야 함)
    loop{ //무한 루프를 사용하여 사용자가 올바른 숫자를 입력할 때까지 계속해서 입력을 받음
        println!("Please input your guess.");

        let mut guess = String::new(); //변수 선언(mut을 통해서 가변성 설정)

        io::stdin() //라이브러리 기능
            .read_line(&mut guess) //입력받은 값을 guess에 저장
            .expect("Failed to read line"); //입력받은 값이 문자열이 아니거나 하는 경우 프로그램이 패닉이 일어나는데, expect는 패닉이 일어날 때 보여줄 메시지를 설정하는 메서드

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); //입력받은 문자열에서 공백을 제거하고, 숫자로 변환하여 guess 변수에 저장. 만약 변환이 실패하면 "Please type a number!" 메시지를 출력하며 패닉이 일어남
        let guess: u32 = match guess.trim().parse() { //입력받은 문자열에서 공백을 제거하고, 숫자로 변환하여 guess 변수에 저장. 만약 변환이 실패하면 루프의 처음으로 돌아감
            Ok(num) => num, //변환이 성공한 경우 num을 반환
            Err(_) => continue //변환이 실패한 경우 루프의 처음으로 돌아감
        };

        println!("You guessed: {guess}"); //입력받은 값을 출력

        match guess.cmp(&secret_number) { //입력받은 값과 랜덤 숫자를 비교하여 결과를 출력
            Ordering::Less => println!("Too small!"), //입력값이 랜덤 숫자보다 작은 경우
            Ordering::Greater => println!("Too big!"), //입력값이 랜덤 숫자보다 큰 경우
            Ordering::Equal => {
                println!("You win!"); //입력값이 랜덤 숫자와 같은 경우
                break; //루프를 종료
            }
        }
    }
    

}
