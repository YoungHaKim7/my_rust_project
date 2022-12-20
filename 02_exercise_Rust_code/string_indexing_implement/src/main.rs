fn main() {
    let some_str = String::from("Hello. \n 안녕하세요");
    let some_vec = some_str.chars().collect::<Vec<char>>();
    for i in 0..some_vec.len() {
        println!("{}", some_vec[i]);
    }
}

// 출처 :
// https://blog.naver.com/littlelib/222697048836
