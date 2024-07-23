mod twiddler5;
mod twiddler6;

fn main() -> std::io::Result<()> {
    //let config = twiddler6::parse()?;

    //println!("{:?}", config);

    //twiddler6::write(config)?;
    twiddler5::parse()?;

    Ok(())
}
