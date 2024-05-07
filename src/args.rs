use clap::{ArgGroup, Args, Parser};

/// Words - tools to manipulate strings on a word-by-word basis.
#[derive(Debug, Clone, Parser)]
#[command(version, author, about)]
#[command(group=ArgGroup::new("log").args(["verbose", "quiet"]).multiple(false))]
#[command(group=ArgGroup::new("from").args(["first", "last"]).multiple(false))]
#[command(group=ArgGroup::new("case").arg("case_sensitive").requires("contains"))]
pub struct WordsArgs {
    /// The source file to read from. If not provided, read from stdin.
    #[arg(short, long)]
    pub source: Option<String>,

    /// The destination file to write to. If not provided, write to stdout.
    #[arg(short, long)]
    pub destination: Option<String>,

    /// Enable verbose logging.
    #[arg(short, long)]
    pub verbose: bool,

    /// Suppress all informational output.
    /// Errors will still be printed to stderr.
    #[arg(short, long)]
    pub quiet: bool,

    /// Get the first n words from the input. (default 1)
    #[arg(short, long)]
    pub first: Option<Option<u64>>,

    /// Get the last n words from the input. (default 1)
    #[arg(short, long)]
    pub last: Option<Option<u64>>,

    /// Filter the input to only include words that contain the given substring.
    #[arg(short, long)]
    pub contains: Option<String>,

    /// Case-sensitive matching.
    #[arg(short = 'C', long)]
    pub case_sensitive: bool,

    /// Work with entire sentences instead of individual words.
    /// Sentences are split on periods.
    /// The output will always end with a period.
    /// Words are split on whitespace.
    #[arg(short = 'S', long)]
    pub sentence: bool,

    #[command(flatten)]
    pub output: Output,
}

#[derive(Debug, Clone, Args)]
#[command(group=ArgGroup::new("format").args(["list", "json"]).multiple(false))]
#[command(group=ArgGroup::new("counting").arg("count").conflicts_with_all(["list", "json", "no_punctuation", "trim", "from"]))]
pub struct Output {
    /// Print the result as a list separated by newlines.
    #[arg(short = 'L', long)]
    pub list: bool,

    /// Print the result as a json list.
    #[arg(short = 'j', long)]
    pub json: bool,

    /// Remove punctuation from the output.
    #[arg(short = 'p', long)]
    pub no_punctuation: bool,

    /// Trim whitespace from the output.
    #[arg(short = 't', long)]
    pub trim: bool,

    /// Count the number of words in the output.
    #[arg(short = 'n', long)]
    pub count: bool,
}