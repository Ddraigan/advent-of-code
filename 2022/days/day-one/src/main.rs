use std::fs;

fn main() {
    let most_calories = find_most_calories(get_sum_of_calories_per_elf(clean_puzzle_input(
        get_puzzle_input(),
    )));
    println!("The Most Calories Is: {:?}", most_calories);

    let top_three_elves_calories = find_three_elves_with_most_calories(
        get_sum_of_calories_per_elf(clean_puzzle_input(get_puzzle_input())),
    );

    let top_three_elves_calories: usize = top_three_elves_calories.iter().sum();
    println!(
        "The Sum of Top 3 Most Calories Is: {:?}",
        top_three_elves_calories
    )
}

fn get_puzzle_input() -> String {
    fs::read_to_string("../../../inputs/day_one.txt").unwrap()
}

fn find_three_elves_with_most_calories(list: Vec<usize>) -> Vec<usize> {
    let mut new_list = list;
    new_list.sort_by(|a, b| a.cmp(b).reverse());
    new_list.truncate(3);

    new_list
}

fn find_most_calories(arr: Vec<usize>) -> usize {
    let mut largest = arr[0];
    let mut i: usize = 0;

    while i < arr.len() {
        if largest < arr[i] {
            largest = arr[i]
        }
        i = i + 1
    }
    largest
}

fn clean_puzzle_input(content: String) -> Vec<String> {
    let split_contents = content.split("\n\r").collect::<Vec<_>>();
    let mut new_split_contents: Vec<String> = vec![];

    for string in split_contents {
        new_split_contents.push(string.replace("\n", " ").trim_start().replace("\r", ""))
    }

    new_split_contents
}

fn get_sum_of_calories_per_elf(arr: Vec<String>) -> Vec<usize> {
    let mut group_of_numbers: Vec<usize> = vec![];

    for item in arr {
        let mut new_num_group: Vec<usize> = vec![];
        let split_item = item.split(" ");
        for item in split_item {
            new_num_group.push(item.parse::<usize>().unwrap())
        }
        group_of_numbers.push(new_num_group.iter().sum())
    }

    group_of_numbers
}
