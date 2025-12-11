use fancy_regex::Regex;

pub fn solve() -> Result<(), anyhow::Error> {
    let input: Vec<(i64, i64)> = std::fs::read_to_string("src/day2/input.txt")?
        .split(",")
        .map(|s| s.trim_ascii())
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();


    println!("day 2 part 1: {}", count(Regex::new(r"^(\d+)(\1)$")?, &input)?);
    println!("day 2 part 2: {}", count(Regex::new(r"^(\d+)(\1)+$")?, &input)?);

    Ok(())
}

fn count(re: Regex, input: &[(i64, i64)]) -> Result<i64, fancy_regex::Error> {
    let mut sum = 0;

    for &(first, last) in input {
        for i in first..=last {
            if re.is_match(&i.to_string())? {
                sum += i;
            }
        }
    }

    Ok(sum)
}
