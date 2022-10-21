use clap::Parser;
use rand_distr::NormalError;
use std::io::stdout;

mod cli {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    // #[clap(name = "td")]
    // #[clap(bin_name = "td")]
    pub struct Cli {
        /// Mean of Normal Distribution
        #[clap(short, long, value_parser)]
        pub mean: f64,

        /// Standard Deviation of Normal Distribution
        #[clap(short, long = "std", value_parser)]
        pub std_dev: f64,

        /// Number of values to be generated
        #[clap(short, long, value_parser)]
        pub num: usize,

        /// Print generated data to STDOUT
        #[clap(short, long, value_parser, default_value_t = false)]
        pub print: bool,

        /// Seed value for PRNG
        #[clap(long, value_parser)]
        pub seed: Option<u64>,

        /// Endianess of output
        #[clap(short, value_enum, long, default_value_t = Endianess::Native)]
        pub endianess: Endianess,

        /// Datatype of output
        #[clap(short, value_enum, long, default_value_t = Datatype::Float)]
        pub datatype: Datatype,
    }

    #[derive(clap::ValueEnum, Clone, Debug)]
    pub enum Endianess {
        #[clap(alias = "be", alias = "b")]
        Big,
        #[clap(alias = "le", alias = "l")]
        Little,
        #[clap(alias = "ne", alias = "n")]
        Native,
    }

    impl Endianess {
        pub fn to_datapool_endianess(&self) -> datapool::Endianess {
            match self {
                Endianess::Big => datapool::Endianess::Big,
                Endianess::Little => datapool::Endianess::Little,
                Endianess::Native => datapool::Endianess::Native,
            }
        }
    }

    #[derive(clap::ValueEnum, Clone, Debug)]
    pub enum Datatype {
        #[clap(alias = "f32", alias = "f")]
        Float,
        #[clap(alias = "f64", alias = "d")]
        Double,
    }
}

fn main() -> Result<(), NormalError> {
    let args = cli::Cli::parse();
    let mut output = stdout();

    match args.datatype {
        cli::Datatype::Double => {
            let values = datapool::get_normal_distribution_f64(
                args.mean,
                args.std_dev,
                args.num,
                args.seed,
            )?;
            if args.print {
                println!("{:?}", values)
            } else {
                let endian = args.endianess.to_datapool_endianess();
                datapool::write_to_disk_f64(&values, &mut output, endian).unwrap();
            }
        }
        cli::Datatype::Float => {
            let values = datapool::get_normal_distribution_f32(
                args.mean as f32,
                args.std_dev as f32,
                args.num,
                args.seed,
            )?;
            if args.print {
                println!("{:?}", values)
            } else {
                let endian = args.endianess.to_datapool_endianess();
                datapool::write_to_disk_f32(&values, &mut output, endian).unwrap();
            }
        }
    }
    Ok(())
}
