// 1. 기본 enum: 신호등
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 2. 값을 담는 enum: 동전
enum Coin {
    Ten,
    Fifty,
    Hundred,
    FiveHundred,
}

fn coin_value(coin: Coin) -> u32 {
    match coin {
        Coin::Ten => 10,
        Coin::Fifty => 50,
        Coin::Hundred => 100,
        Coin::FiveHundred => 500,
    }
}

// 3. Option은 사실 Rust 내장 enum
// enum Option<T> {
//     Some(T),  <- 값이 있음
//     None,     <- 값이 없음
// }

// Option처럼 데이터를 담는 enum 직접 만들어보기
enum MyMessage {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 구조체처럼 이름 있는 필드
    Write(String),              // 문자열 하나
    ChangeColor(u8, u8, u8),    // 숫자 세 개 (RGB)
}

fn handle_message(msg: MyMessage) {
    match msg {
        MyMessage::Quit => println!("종료"),
        MyMessage::Move { x, y } => println!("({x}, {y})로 이동"),
        MyMessage::Write(text) => println!("메시지: {text}"),
        MyMessage::ChangeColor(r, g, b) => println!("색상 변경: R={r} G={g} B={b}"),
    }
}

fn main() {
    // 1. match로 enum 분기
    let light = TrafficLight::Red;
    match light {
        TrafficLight::Red => println!("정지!"),
        TrafficLight::Yellow => println!("준비!"),
        TrafficLight::Green => println!("출발!"),
    }

    // 2. 동전 값 계산
    let coin = Coin::Hundred;
    println!("동전 가치: {}원", coin_value(coin));

    // 3. 데이터를 담는 enum
    handle_message(MyMessage::Move { x: 10, y: 20 });
    handle_message(MyMessage::Write(String::from("안녕하세요")));
    handle_message(MyMessage::ChangeColor(255, 128, 0));
    handle_message(MyMessage::Quit);

    // 4. Option: 값이 있을 수도, 없을 수도 있을 때
    let some_number: Option<i32> = Some(7);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("숫자가 있어요: {n}"),
        None => println!("숫자가 없어요"),
    }

    match no_number {
        Some(n) => println!("숫자가 있어요: {n}"),
        None => println!("숫자가 없어요"),
    }

    // 5. if let: 특정 경우만 처리하고 싶을 때
    if let Some(n) = some_number {
        println!("if let으로 꺼낸 값: {n}");
    }

    // 6. unwrap_or: None일 때 기본값 지정
    let val = no_number.unwrap_or(0);
    println!("값이 없으면 기본값 사용: {val}");
}
