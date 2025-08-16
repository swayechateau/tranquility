use tranquility::run_cli;

fn main() -> anyhow::Result<()> {
    match run_cli() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e); 
            std::process::exit(1);
        }
    }
}