fn main() {
    let ln1 = vec![7,6,4,2,1];
    let ln2 = vec![1,2,7,8,9];
    let ln3 = vec![9,7,6,2,1];
    let ln4 = vec![1,3,2,4,5];
    let ln5 = vec![8,6,4,4,1];
    let ln6 = vec![1,3,6,7,9];
    let ln7 = vec![
9,12,14,16,17,18,15];
    let example = vec![ln1, ln2, ln3, ln4, ln5, ln6, ln7];
    let lines = lines_from_file("src/input.txt").expect("Useable File");

    let mut example_count = 0;

    for line in example.iter() {
        if check_safety(line) == Safety::Safe {
            example_count += 1
        }
    }

    let mut count = 0;

    for line in lines.iter() {
        let linex: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let slice = &linex;
        if check_safety(slice) == Safety::Safe {
            count += 1
        }
    }

    println!("{}", count);
    println!("{}", example_count)
}

#[derive(PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(PartialEq, Clone, Copy)]
enum Level {
    Increment,
    Decrement,
    Unset,
}

fn check_diff(a: i32,b: i32) -> Safety
{

    let x = std::cmp::max(a, b);
    let y = std::cmp::min(a, b);

    let diff = x-y;

    if diff > 3 {
        return Safety::Unsafe;
    }

    if diff < 1 {
        return Safety::Unsafe;
    }

    return Safety::Safe;
}


fn check_safety(slice: &[i32]) -> Safety
{
    let mut slice_peak = slice.into_iter().peekable();
    let mut direction = Level::Unset;
    let mut unsafe_counter = 0;

    for _ in 0..slice.len() {
        let num = slice_peak.next();
        let next_num = slice_peak.peek().copied();
        let mut next_num_mut = slice_peak.peek_mut();

        if next_num_mut.is_none() {
            break;
        }

        if unsafe_counter > 1 {
            return Safety::Unsafe
        }

        let a = *num.unwrap();
        let b = next_num.cloned().unwrap();

        if check_diff(a,b) == Safety::Unsafe {
            next_num_mut.take();
            unsafe_counter += 1;
        }

        let current_direction = check_up_or_down(a, b);

        if direction == Level::Unset {
            direction = current_direction
        }

        if current_direction != direction {
            next_num_mut.take();
            unsafe_counter += 1;
        }
    };

    return Safety::Safe;
}

fn check_up_or_down(a: i32, b: i32) -> Level
{
    if a > b {
        return Level::Decrement
    }
    return Level::Increment
}

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let buf = std::io::BufReader::new(file);
    let line = std::io::BufRead::lines(buf).collect();
    line
}
