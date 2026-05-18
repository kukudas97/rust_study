use crate::garden::vegetables::Asparagus; //use를 이용하여 Asparagus를 가져옴

pub mod garden; //garden 모듈을 가져옴

fn main() {
    let plant = Asparagus {};

    println!("{plant:?}"); 
}