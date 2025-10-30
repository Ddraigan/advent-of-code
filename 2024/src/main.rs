fn main() {
    let input = lines_from_file("src/input.txt").expect("Useable input file");
    let mut answers = vec![];

    let mut left_lists = vec![];
    let mut right_lists = vec![];


    for line in input.iter() {
        let mut string_split = line.split_whitespace();
        let mut a: Vec<usize> = string_split.next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut b: Vec<usize> = string_split.next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

        a.sort_unstable();
        b.sort_unstable();

        left_lists.push(a);
        right_lists.push(b);
    }


    assert_eq!(left_lists.len(), right_lists.len());

    for (i, list) in left_lists.iter().enumerate() {
        for (ii, lnum) in list.iter().enumerate() {
            let rnum = right_lists[i][ii];

            let a = std::cmp::min(*lnum, rnum);
            let b = std::cmp::max(*lnum, rnum);

            answers.push(b-a);
        }
    }

    println!("{:?}", answers);
    println!("{}", answers.iter().sum::<usize>())
}

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf).collect()
}
