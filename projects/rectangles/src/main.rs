// 해당부분은 구조체를 사용하여 구조체 이해를 위한 예시입니다. 
// 구조체는 여러 개의 관련된 값을 하나의 단위로 묶어서 표현할 수 있는 사용자 정의 데이터 타입입니다.
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     /*
//     let width1 = 30;
//     let height1 = 50;
//     */
//     /*
//     let rect1 = (30, 50);
//     */
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//        // area(width1, height1)
//         area(&rect1)
//     );
// }

// fn area(/*width: u32, height: */ /*dimensions : (u32, u32)*/ rectangle: &Rectangle) -> u32 {
//     //width * height
//     //dimensions.0 * dimensions.1
//     rectangle.width * rectangle.height
// }


// 해당부분은 구조체에 메서드를 추가하여 구조체 이해를 위한 예시입니다.
// self는 메서드가 호출된 인스턴스를 가리키는 특별한 매개변수입니다.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// 해당부분은 구조체에 메서드를 추가하여 구조체 이해를 위한 예시입니다.
// can_hold 메서드는 self가 다른 Rectangle 인스턴스를 포함할 수 있는지 여부를 판단하는 메서드입니다. 
// impl 블록은 여러개의 메서드를 Rectangle 구조체에 추가할 수 있습니다.
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}