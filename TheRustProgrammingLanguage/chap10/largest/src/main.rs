

fn largest(list:&[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest; // 还是带return顺眼
}



// 泛型版本
// fn largest_2<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);


    //
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_2(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_2(&char_list);
    // println!("The largest char is {}", result);
}