use std::io;

fn main() -> io::Result<()> {
    let mut i = 1;
    let num = loop {
        let mut input = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin().read_line(&mut input)?;
        if input.trim() == "The letter e" {
            break i;
        } else {
            i += 1
        }
    };
    println!("It took you {} trials to get the right answer", num);

    Ok(())
}
