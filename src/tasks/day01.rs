fn get_vecs(data: String) -> (Vec<i32>, Vec<i32>) {
    let mut first_numbers: Vec<i32> = Vec::new();
    let mut second_numbers: Vec<i32> = Vec::new();

    for line in data.lines() {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();

        if let Some(number) = numbers.get(0) {
            first_numbers.push(number.parse::<i32>().unwrap());
        }
        if let Some(number) = numbers.get(1) {
            second_numbers.push(number.parse::<i32>().unwrap());
        }
    }

    (first_numbers, second_numbers)
}

#[test]
fn test1() {
    let result = solve1(String::from("3   4
4   3
2   5
1   3
3   9
3   3"));
    assert_eq!(result, 11);
}

pub fn solve1(data: String) -> i32 {
    let (mut first_numbers, mut second_numbers) = get_vecs(data);

    first_numbers.sort();
    second_numbers.sort();

    first_numbers.iter().zip(second_numbers.iter()).map(|(a, b)| (a - b).abs()).sum()
}

#[test]
fn test2() {
    let result = solve2(String::from("3   4
4   3
2   5
1   3
3   9
3   3"));
    assert_eq!(result, 31);
}

pub fn solve2(data: String) -> i32 {
    let (first_numbers, second_numbers) = get_vecs(data);

    first_numbers.iter().map(|a| {
        let count = second_numbers.iter().fold(0, |res, x| if x == a { res + 1 } else { res } );
        count * a
    }).sum()
}
