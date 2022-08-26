use core::panic;

/// this is a basic struct used for basically storing and printing the contents of each row in the pascal triangle.
/// 
/// the row gets created with a buffer that has every value set to 0.
/// 
/// the row implements Display so that the Row can be printed in a drawing function.
#[derive(Clone, Debug)]
struct Row{ pub buffer : Vec<u128> }
impl Row {
    fn new(row_count : usize) -> Row { Row{ buffer : vec![0; row_count as usize] } }
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

/// this function will draw pascals triangle out to a given row
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
                // the first number should always be a one and every number after that in the next row should be the top two numbers above it added together
                0 => { current_row.buffer[j as usize] = 1 }
                _ => { current_row.buffer[j as usize] = previous_row.buffer[j as usize] + previous_row.buffer[j as usize - 1] }
            }
        }
        println!("{}", current_row);
        previous_row = current_row.clone()
    }
}

/// this function will get a u128 from cmd args
fn get_cmd_arg_row_count() -> u128 {
    const MAX_PASCAL_TRIANGLE_SIZE : u128 = 132;

    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("error no pascal triangle size argument given");
    }

    // there should only be one argument given for the amount o
    let parse_result = args[1].parse::<u128>();
    
    let number = match parse_result {
        Ok(parsed_number) => parsed_number,
        Err(_error) => panic!("error invalid argument given you have to put in a number")
    };

    if number > MAX_PASCAL_TRIANGLE_SIZE {
        panic!("error number too large pick a size equal to or smaller than {}", MAX_PASCAL_TRIANGLE_SIZE);
    }

    number
}

fn main() {
    draw_pascals_triangle(get_cmd_arg_row_count());
}