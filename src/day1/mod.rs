use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> Result<(), anyhow::Error> {
    let file = File::open("src/day1/input.txt")?;
    let buf = BufReader::new(file);
    let steps = buf
        .lines()
        .map(|r| -> Result<(u8, i32), anyhow::Error> {
            let line = r?;
            let (dir, num) = line.split_at(1);
            Ok((dir.as_bytes()[0], num.parse()?))
        })
        .collect::<Result<Vec<_>, _>>()?;

    part1(&steps)?;
    part2(&steps)?;

    Ok(())
}

fn part1(steps: &[(u8, i32)]) -> Result<(), anyhow::Error> {
    let mut password = 0;
    let mut dial = 50;

    for (dir, num) in steps {
        match dir {
            b'L' => dial += num,
            b'R' => dial -= num,
            _ => unreachable!(),
        }
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }

    println!("day 1 part 1: {password}");

    Ok(())
}

fn part2(steps: &[(u8, i32)]) -> Result<(), anyhow::Error> {
    let mut password = 0;
    let mut dial = 50;

    for &(dir, num) in steps {
        let sign = match dir {
            b'L' => 1,
            b'R' => -1,
            _ => unreachable!(),
        };

        for _ in 0..num {
            dial += sign;
            dial %= 100;
            if dial == 0 {
                password += 1;
            }
        }
    }

    println!("day 1 part 2: {password}");

    Ok(())
}
