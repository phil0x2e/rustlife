use clap::Parser;

/// Game Of Life for the terminal implemented in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of columns of the grid
    #[arg(long, default_value_t = 50)]
    width: u32,
    /// Number of rows of the grid
    #[arg(long, default_value_t = 50)]
    height: u32,
    /// Number of milliseconds between iterations
    #[arg(short, long, default_value_t = 200)]
    tick: u32,
    /// String that is displayed for alive cells
    #[arg(short, long, default_value = "██")]
    alive: String,
    /// String that is displayed for dead cells
    #[arg(short, long, default_value = "  ")]
    dead: String,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}")
}
