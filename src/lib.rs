pub struct Config {
    pub file_path: String,
    pub format: String,
    pub dest_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 4 {
            return Err("3 arguments required");
        }

        let file_path = args[1].clone();
        let format = args[2].clone();
        let dest_path = args[3].clone();

        Ok(Config { file_path, format, dest_path})
    }
}
