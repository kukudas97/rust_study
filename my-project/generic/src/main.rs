// fn largest(list: &[i32]) -> &i32 {//largest 함수는 i32 타입의 슬라이스를 받아서 가장 큰 값을 반환하는 함수입니다.
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {//main 함수에서는 largest 함수를 사용하여 두 개의 숫자 리스트에서 가장 큰 값을 찾습니다.
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {//Point 구조체에 대한 제네릭 구현입니다. mixup 메서드는 두 개의 Point 인스턴스를 받아서 새로운 Point를 반환합니다. 새로운 Point의 x 값은 self의 x 값이고, y 값은 other의 y 값입니다.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {//main 함수에서는 Point 구조체를 사용하여 두 개의 포인트를 만들고, mixup 메서드를 사용하여 새로운 포인트를 만듭니다.
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}