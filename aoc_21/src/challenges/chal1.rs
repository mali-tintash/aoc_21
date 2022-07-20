
use aoc_21::read_lines;


enum FileError
{
    FileError1
}


pub fn larger_measurements_count(path: &str) -> i32 {
    //let f = fs::read_to_string(path).unwrap(); 
    let mut p1 = -1;
    let mut p2 = -1;
    let mut biggerCount = 0;


    if let Ok(lines) = read_lines(path){
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