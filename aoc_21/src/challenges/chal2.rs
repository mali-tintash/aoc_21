
pub fn larger_window_count(_path: &str) -> usize {

    

    let bigger_window = include_str!("../../chal1_input.txt")
                    .lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u16>>()
                    .windows(3)
                    .map(|x| x[0] + x[1] + x[2])
                    .collect::<Vec<u16>>()
                    .windows(2)
                    .filter(|x| x[0] < x[1])
                    .count();

     //let p = bigger_window.windows(2);
    //         .windows(3);

    // let windows = p.clone();
    // for w in p.into_iter() {
    //     println!("{:?} ->",w);
    // }

    //let sum_of_windows : Vec<u16> = windows.map(|x| x[0] + x[1] + x[2]).collect();

   
    
    return bigger_window;
}

// pub fn get_int_from_result(input: Option<Result<String,std::io::Error>>) -> i32{

//     if let Some(el) = input {
//         el.unwrap().parse::<i32>().unwrap();
//     }
//     return 0;
// }