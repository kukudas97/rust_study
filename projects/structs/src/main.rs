struct User { // 구조체 정의
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User { // 구조체 인스턴스 생성
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
   };

    let _user2 = User { // 구조체 업데이트 구문 (수동으로 필드 복사)
        active: user1.active,
        username: user1.username.clone(), // clone()으로 소유권 유지
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let _user3 = User { // 구조체 업데이트 구문 (더 간단한 방법: .. 문법)
        email: String::from("another@example.com"),
        ..user1 // user1의 나머지 필드를 그대로 사용
    };
}
