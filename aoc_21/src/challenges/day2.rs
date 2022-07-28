pub fn calculate_pos_depth() -> i32 {

    let mut hor_pos = 0;
    let mut depth = 0;

    let output = include_str!("../../day2_input.txt")
                .lines()
                .map(|l| l.split_once(" ").unwrap())
                .fold((0,0) , |(f,d),(s,i)| {
                    match (s,i.parse::<i32>().unwrap()) {
                        ("forward",i) => (f+i,d),
                        ("down",i) => (f,d+i),
                        ("up",i) => (f,d-i),
                        _ => unreachable!(),
                    }
                });
                //.map()

    //println!("{}",output.0 * output.1);
    return output.0 * output.1;
}