use std::{env, error::Error, fs, process};

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("The arguments is not enough");
        }

        args.next();
        let query = args.next().unwrap();
        let filename = args.next().unwrap();
        Ok(Config { query, filename })
    }
}

fn output(query: &str, content: &str) {
    let result: Vec<&str> = content
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    if result.len() > 0 {
        println!("There are {} matching results", result.len());
        for i in result {
            println!("{}", i)
        }
    } else {
        println!("There is no matching results")
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    output(&config.query, &content);
    Ok(())
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
