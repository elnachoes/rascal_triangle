use core::panic;

const MAX_PASCAL_TRIANGLE_SIZE : u128 = 132;

#[derive(Clone, Debug)]
struct Row{ pub buffer : Vec<u128> }
impl Row {
    fn new(row_count : usize) -> Row {
        Row{ buffer : vec![0; row_count as usize]}
    }
}
impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.buffer.iter() {
            if *i == 0 {
                write!(f, "  ")?;
            } else {
                write!(f, "{} ", i)?;
            }
        }
        Ok(())
    }
}

fn draw_pascals_triangle(row_count : u128) {
    let mut previous_row = Row::new(row_count as usize);
    let mut current_row : Row;

    // manually do out the first two rows because they are all 1
    previous_row.buffer[0] = 1;
    println!("{}", previous_row);
    if row_count == 1 { return }

    previous_row.buffer[1] = 1;
    println!("{}", previous_row);
    if row_count == 2 { return }
    
    // skip the first two rows because they were done just above
    for _ in 0..row_count - 2  {
        current_row = Row::new(row_count as usize);
        for j in 0..row_count {
            match j {
                //the first number should always be a one and every number after that in the next row should be the top two numbers above it added together
                0 => { current_row.buffer[j as usize] = 1 }
                _ => { current_row.buffer[j as usize] = previous_row.buffer[j as usize] + previous_row.buffer[j as usize - 1] }
            }
        }
        println!("{}", current_row);
        previous_row = current_row.clone()
    }
}

fn main() {
    //parse cmd args for a pascal row count
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("error no pascal triangle size argument given");
    }

    let parse_result = args[1].parse::<u128>();
    
    let number = match parse_result {
        Ok(parsed_number) => parsed_number,
        Err(_error) => panic!("error invalid number")
    };

    if number > MAX_PASCAL_TRIANGLE_SIZE {
        panic!("error number too large pick a size equal to or smaller than {}", MAX_PASCAL_TRIANGLE_SIZE);
    }

    draw_pascals_triangle(number);
}