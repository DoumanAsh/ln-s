use arg::Args;

#[derive(Args, Debug)]
///Find files utility
pub struct Cli {
    #[arg(short, long)]
    ///Automatically over-writes existing link, if any. By default exits with error.
    pub force: bool,
    #[arg(short, long)]
    ///Uses absolute path, when creating sylink
    pub abs: bool,
    #[arg(required)]
    ///Specifies name of symlink. Can be full path.
    pub link: String,
    #[arg(required)]
    ///Specifies path, which to link
    pub path: String,
}

impl Cli {
    pub fn new<'a, T: IntoIterator<Item = &'a str>>(args: T) -> Result<Self, i32> {
        let args = args.into_iter().skip(1);

        Cli::from_args(args).map_err(|err| match err.is_help() {
            true => {
                println!("{}", Cli::HELP);
                0
            },
            false => {
                eprintln!("{}", err);
                2
            },
        })
    }
}
