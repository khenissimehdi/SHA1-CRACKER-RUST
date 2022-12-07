use std:: {
    error::Error,
    env, fs::File, io::{BufReader, BufRead}
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlistFile = File::open(&args[1])?;
    let reader = BufReader::new(&wordlistFile);
    
    for line in reader.lines()  {
        let line = line?.trim().to_string();
        println!("{}",line);
    }
    Ok(())

}
