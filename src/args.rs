use clap::{ArgGroup, Parser};

/// Words - tools to manipulate strings on a word-by-word basis.
#[derive(Debug, Clone, Parser)]
#[command(version, author, about)]
#[command(group=ArgGroup::new("log").multiple(false))]
#[command(group=ArgGroup::new("from").multiple(false))]
#[command(group=ArgGroup::new("case").requires("contains"))]
#[command(group=ArgGroup::new("format").multiple(false))]
pub struct Args {
    /// The source file to read from. If not provided, read from stdin.
    #[arg(short, long)]
    pub source: Option<String>,

    /// The destination file to write to. If not provided, write to stdout.
    #[arg(short, long)]
    pub destination: Option<String>,

    /// Enable verbose logging.
    #[arg(short, long, group = "log")]
    pub verbose: bool,

    /// Suppress all informational output.
    /// Errors will still be printed to stderr.
    #[arg(short, long, group = "log")]
    pub quiet: bool,

    /// Get the first n words from the input. (default 1)
    #[arg(short, long, group = "from")]
    pub first: Option<Option<u64>>,

    /// Get the last n words from the input. (default 1)
    #[arg(short, long, group = "from")]
    pub last: Option<Option<u64>>,

    /// Filter the input to only include words that contain the given substring.
    #[arg(short, long)]
    pub contains: Option<String>,

    /// Case-sensitive matching.
    #[arg(short = 'C', long, group = "case")]
    pub case_sensitive: bool,

    /// Work with entire sentences instead of individual words.
    /// Sentences are split on periods.
    /// The output will always end with a period.
    /// Words are split on whitespace.
    #[arg(short = 'S', long)]
    pub sentence: bool,

    /// Print the result as a list seperated by newlines.
    #[arg(short = 'L', long, group = "format")]
    pub list: bool,

    /// Print the result as a json list.
    #[arg(short = 'j', long, group = "format")]
    pub json: bool,
}