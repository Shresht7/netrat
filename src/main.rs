use netrat::connection;

mod cli;
mod commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse the command-line arguments
    let cli = cli::parse();

    // Run the application
    cli.run()?;

    Ok(())
}
