use clap::Parser;
use log::{debug, info};

#[derive(Parser)]
pub struct MinRootFifthVestaArgs {
    #[arg(long, short = 'n', value_name = "N", help = "Number of iterations")]
    pub n: u64,

    #[arg(long, short = 'x', value_name = "X", help = "Initial input X")]
    pub x: u64,

    #[arg(long, short = 'y', value_name = "Y", help = "Initial input Y")]
    pub y: u64,
}

#[derive(Parser)]
#[command(name = "minroot-fifth", version = "0.1.0", about = "MinRoot VDF")]
pub enum Commands {
    #[command(name = "minroot-fifth")]
    MinRootFifthVesta(MinRootFifthVestaArgs),
}

pub fn main() {
    // See https://github.com/rust-lang/log
    env_logger::init();

    let args = Commands::parse();
    match args {
        Commands::MinRootFifthVesta(args) => {
            let x = ark_vesta::Fr::from(args.x);
            let y = ark_vesta::Fr::from(args.y);
            let n = args.n;
            info!(
                "Computing MinRoot VDF with x = {:?}, y = {:?}, iterating {:?} times",
                x, y, n
            );
            let mut final_res = (x, y);
            let start_time = std::time::Instant::now();
            (0..n).for_each(|i| {
                let res = vdf_rs::minroot::fifth(final_res.0, final_res.1, i as usize);
                final_res = res;
                debug!("Results at step {i}: {:?}", final_res);
            });
            let elapsed = start_time.elapsed();
            info!(
                "Elapsed time: {} μs, {} ms, {} s",
                elapsed.as_micros(),
                elapsed.as_millis(),
                elapsed.as_secs()
            );
        }
    }
}
