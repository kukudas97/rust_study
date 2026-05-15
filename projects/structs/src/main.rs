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

    let user2 = User { // 구조체 업데이트 구문
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User { // 구조체 업데이트 구문 (더 간단한 방법)
        email: String::from("another@example.com"),
        ..user1
    };
}
