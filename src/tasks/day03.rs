use regex::Regex;

#[test]
fn test1() {
    let result = solve1(
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    );
    assert_eq!(result, 161);
}

pub fn solve1(data: String) -> i32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();

    regex
        .captures_iter(&data)
        .map(|caps| caps[1].parse::<i32>().expect("meow") * caps[2].parse::<i32>().expect("meow"))
        .sum()
}

#[test]
fn test2() {
    let result = solve2(String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
    assert_eq!(result, 48);
}

pub fn solve2(data: String) -> i32 {
    let pattern = r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))";
    let regex = Regex::new(pattern).unwrap();

    let mut enabled = true;

    regex
        .captures_iter(&data)
        .map(|caps| {
            match &caps[1] {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        return caps[2].parse::<i32>().expect("meow") * caps[3].parse::<i32>().expect("meow");
                    }
                }
            }
            0
        })
        .sum()
}
