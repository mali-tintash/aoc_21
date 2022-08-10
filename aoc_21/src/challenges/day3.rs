pub fn calculate_gama_epsilon(){

    let lines = include_str!("../../day3_input_test.txt")
                .lines()
                .collect::<Vec<&str>>();

    let mut a = [0,0,0,0,0];

    let total_lines = lines.len();
    

    for line in lines {
        for (i,cc) in line.chars().enumerate(){
            if cc == '1' {
                a[i] += 1;
            }
        }
    }

    let gama = calculate_gama(a.to_vec(),total_lines);
    let epsilon = calculate_epsilon(&gama);

    println!("Gama : {:?} | Eps : {:?}",gama,epsilon);
}

pub fn calculate_gama(input: Vec<i8>, len : usize)-> Vec<i8>{

    let mut gama = vec![0,0,0,0,0];
    let half_total_lines : i8 = (len/2) as i8;

    for (idx,i) in input.iter().enumerate(){
        if *i > half_total_lines {
            gama[idx as usize] = 1;
        }
    }
    
    gama
}

pub fn calculate_epsilon(input : &Vec<i8>) -> Vec<i8>{

    let mut epsilon: Vec<i8> = vec![0,0,0,0,0];

    for (idx,i) in input.iter().enumerate() {
        
        if *i == 0 {
            epsilon[idx] = 1;
        }
    };

    epsilon
}