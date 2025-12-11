mod day1;
mod day2;

fn main() -> Result<(), anyhow::Error> {
    day1::solve()?;
    day2::solve()?;

    Ok(())
}
