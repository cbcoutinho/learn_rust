pub struct Config {
    pub filename_in: String,
    pub filename_out: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename_in = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename_in string"),
        };

        let filename_out = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename_out string"),
        };

        Ok(Config {
            filename_in: filename_in,
            filename_out: filename_out,
        })
    }
}
