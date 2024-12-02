fn parse_data(data: String) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let numbers = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<i32>>();

        vec.push(numbers);
    }

    vec
}

#[test]
fn test1() {
    let result = solve1(String::from("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"));
    assert_eq!(result, 2);
}

pub fn solve1(data: String) -> i32 {
    let days = parse_data(data);

    days.iter().map(|x| {
        let mut prev = x.get(0).unwrap();
        let dir = if x.get(1).unwrap() > prev { 1 } else { -1 };

        x.iter().skip(1).map(|a| {
            let diff = (a - prev).abs();
            if diff < 1 || diff > 3 {
                return 0;
            }
            let curr_dir = if a > prev { 1 } else { -1 };
            if curr_dir - dir != 0 {
                return 0;
            }
            prev = a;
            1
        }).min().unwrap()
    }).sum()
}

#[test]
fn test2() {
    let result = solve2(String::from("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"));
    assert_eq!(result, 4);
}

pub fn solve2(data: String) -> i32 {
    let days = parse_data(data);

    days.iter().map(|x| {
        for i in 0..x.len()+1 {
            let special_x: Vec<i32> = x
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index != i) // Exclude indices 0 and 2
                        .map(|(_, &value)| value) // Extract the values
                        .collect();

            let mut prev = special_x.get(0).unwrap();
            let dir = if special_x.get(1).unwrap() > prev { 1 } else { -1 };
    
            let res = special_x.iter().skip(1).map(|a| {
                let diff = (a - prev).abs();
                if diff < 1 || diff > 3 {
                    return 0;
                }
                let curr_dir = if a > prev { 1 } else { -1 };
                if curr_dir - dir != 0 {
                    return 0;
                }
                prev = a;
                1
            }).min().unwrap();

            if res == 1 {
                return 1;
            }
        }
        0
    }).sum()
}
