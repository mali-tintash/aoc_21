use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

enum FileError
{
    FileError1
}

pub fn larger_measurements_count(path: String) -> i32 {
    //let f = fs::read_to_string(path).unwrap(); 
    let mut p1 = -1;
    let mut p2 = -1;
    let mut biggerCount = 0;

    if let Ok(lines) = read_lines(path.as_str()){
        for line in lines{
            if let Ok(entry) = line {
                
                p1 = entry.parse::<i32>().unwrap();
                if p2 >= 0 {
                    if p1 > p2 {
                        biggerCount += 1;
                    }
                }
                p2 = p1;
                
            }
        }
    }
    

    return biggerCount;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {

    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}