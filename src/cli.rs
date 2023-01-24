use clap::{Parser, ValueEnum};

/// Generate Nix fetcher calls from repository URLs
/// https://github.com/nix-community/nurl
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    /// URL to the repository to be fetched
    #[arg(
        required_unless_present = "command",
        default_value_t, // placeholder, will not be accessed
        hide_default_value = true
    )]
    pub url: String,

    /// The revision or reference to be fetched
    pub rev: Option<String>,

    /// Specify the fetcher function instead of inferring from the URL
    #[arg(short, long)]
    pub fetcher: Option<FetcherFunction>,

    /// Extra indentation (in number of spaces)
    #[arg(short, long, default_value_t = 0)]
    pub indent: usize,

    /// Only output the hash
    #[arg(short = 'H', long, group = "format")]
    pub hash: bool,

    /// Output in json format
    #[arg(short, long, group = "format")]
    pub json: bool,

    /// Parse the url without fetching the hash, output in json format
    ///
    /// Note that --arg(-str) and --overwrite(-str) will be ignored silently
    #[arg(short, long, group = "format")]
    pub parse: bool,

    /// Additional arguments to pass to the fetcher
    #[arg(short, long = "arg", num_args = 2, value_names = ["NAME", "EXPR"])]
    pub args: Vec<String>,

    /// Same as --arg, but accepts strings instead Nix expressions
    #[arg(short = 'A', long = "arg-str", num_args = 2, value_names = ["NAME", "STRING"])]
    pub args_str: Vec<String>,

    /// Overwrite arguments in the final output,
    /// not taken into consideration when fetching the hash
    ///
    /// Note that nurl does not verify any of the overwrites,
    /// for the final output to be valid,
    /// the user should not overwrite anything that would change the hash
    ///
    /// Examples:
    /// {n}  --overwrite repo pname
    /// {n}  --overwrite rev version
    #[arg(short, long = "overwrite", num_args = 2, value_names = ["NAME", "EXPR"])]
    pub overwrites: Vec<String>,

    /// Same as --overwrite, but accepts strings instead Nix expressions
    ///
    /// Examples:
    /// {n}  --overwrite-str rev 'v${version}'
    /// {n}  --overwrite-str meta.homepage https://example.org
    #[arg(short = 'O', long = "overwrite-str", num_args = 2, value_names = ["NAME", "STRING"])]
    pub overwrites_str: Vec<String>,

    /// List all available fetchers
    #[arg(short, long, group = "command")]
    pub list_fetchers: bool,

    /// List all fetchers that can be generated without --fetcher
    #[arg(short = 'L', long, group = "command")]
    pub list_possible_fetchers: bool,

    /// Print out the listed fetchers with the specified separator,
    /// only used when --list-fetchers or --list-possible-fetchers is specified
    #[arg(
        short = 's',
        long,
        value_name = "SEPARATOR",
        allow_hyphen_values = true
    )]
    pub list_sep: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "camelCase")]
pub enum FetcherFunction {
    #[clap(name = "builtins.fetchGit")]
    BuiltinsFetchGit,
    FetchCrate,
    FetchFromBitbucket,
    FetchFromGitHub,
    FetchFromGitLab,
    FetchFromGitea,
    FetchFromGitiles,
    FetchFromRepoOrCz,
    FetchFromSourcehut,
    FetchHex,
    Fetchgit,
    Fetchhg,
    Fetchsvn,
}
