fn main() {
    let input = lines_from_file("src/input.txt").expect("Useable input file");
    let mut answers = vec![];

    let mut left_list = vec![];
    let mut right_list = vec![];


    for line in input.iter() {
        let mut string_split = line.split_whitespace();

        let a = string_split.next().unwrap().parse::<usize>().unwrap();
        let b = string_split.next().unwrap().parse::<usize>().unwrap();

        left_list.push(a);
        right_list.push(b);
    }

    assert_eq!(left_list.len(), right_list.len());

    left_list.sort_unstable();
    right_list.sort_unstable();

    for (i, lnum) in left_list.iter().enumerate() {
            let rnum = right_list[i];

            let a = std::cmp::min(*lnum, rnum);
            let b = std::cmp::max(*lnum, rnum);

            answers.push(b-a);
    }

    println!("{:?}", answers);
    println!("{}", answers.iter().sum::<usize>());

    let mut similarity = vec![];

    for lnum in &left_list {
        let mut count = 0;

        for rnum in &right_list {
            if lnum == rnum {
                count += 1
            }
        }

        similarity.push(count * lnum)
    }

    println!("{}", similarity.iter().sum::<usize>())
}

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf).collect()
}
