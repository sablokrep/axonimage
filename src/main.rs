use std::error::Error;
mod args;
mod bleached;
mod healthy;
mod normalize;
mod rotation;
mod smartcore;
mod smartun;
mod train;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::train::conv2d;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("axonimage");
    println!("{}", repgenerate.unwrap());

    let args = CommandParse::parse();
    match &args.command {
        Commands::Convolutional {
            normal,
            bleached,
            thread,
        } => {
            let n_threads = thread.parse::<usize>().expect("thread must be a number");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let runmodel = conv2d(&normal, &bleached).unwrap();
                println!("The qsar modelling has finished: {}", runmodel);
            });
        }
    }
}
