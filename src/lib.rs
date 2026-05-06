//! # bsd-getopt
//!
//! A small, dependency-free implementation of a BSD-style `getopt` parser in Rust.
//!
//! This crate provides a minimal and predictable way to parse short command-line
//! options (e.g. `-a`, `-b value`, `-abc`) similar to traditional Unix `getopt`.
//!
//! ## Features
//!
//! - Supports grouped options (`-abc`)
//! - Supports options with required arguments (`-o value` or `-ovalue`)
//! - Handles `--` to terminate option parsing
//! - Exposes `optind`, `optarg`, and `optopt` like classic `getopt`
//! - No unsafe code, no dependencies
//!
//! ## Example
//!
//! ```rust
//! use bsd_getopt::Getopt;
//!
//! let args = vec![
//!     "prog".to_string(),
//!     "-a".to_string(),
//!     "-b".to_string(),
//!     "value".to_string(),
//!     "-cfoo".to_string(),
//! ];
//!
//! let mut parser = Getopt::new("ab:c:", args);
//!
//! while let Some(opt) = parser.next() {
//!     match opt {
//!         'a' => println!("option a"),
//!         'b' => println!("option b with arg {:?}", parser.optarg),
//!         'c' => println!("option c with arg {:?}", parser.optarg),
//!         '?' => println!("unknown option: {}", parser.optopt),
//!         ':' => println!("missing argument for: {}", parser.optopt),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ## `optstring` format
//!
//! - `a` → option without argument
//! - `b:` → option requires argument
//! - `:abc` → suppress error messages, return `:` on missing argument
//!
//! ## Notes
//!
//! - Parsing stops at the first non-option argument or `--`
//! - This crate only supports short options (no long options like `--help`)

/// A BSD-style command-line option parser.
///
/// `Getopt` iterates over command-line arguments and returns options one by one,
/// mimicking the behavior of traditional Unix `getopt`.
///
/// The parser maintains internal state such as:
///
/// - `optind`: index of the next argument to process
/// - `optarg`: argument associated with the current option (if any)
/// - `optopt`: the last option character processed
/// - `opterr`: whether to print error messages
///
/// # Example
///
/// ```rust
/// use bsd_getopt::Getopt;
///
/// let args = vec![
///     "prog".to_string(),
///     "-a".to_string(),
///     "-bvalue".to_string(),
/// ];
///
/// let mut g = Getopt::new("ab:", args);
///
/// assert_eq!(g.next(), Some('a'));
/// assert_eq!(g.next(), Some('b'));
/// assert_eq!(g.optarg, Some("value".to_string()));
/// assert_eq!(g.next(), None);
/// ```
pub struct Getopt {
    args: Vec<String>,
    optstring: String,
    pub optind: usize,
    pub optopt: char,
    pub optarg: Option<String>,
    pub opterr: bool,
    nextchar_idx: usize, 
}

impl Getopt {
    /// Creates a new `Getopt` parser.
    ///
    /// # Parameters
    ///
    /// - `optstring`: A string describing valid options.
    /// - `args`: Command-line arguments (typically from `std::env::args()`).
    ///
    /// # Behavior
    ///
    /// - Parsing starts from index 1 (skipping program name)
    /// - Internal state is initialized to default values
    ///
    /// # Example
    ///
    /// ```rust
    /// use bsd_getopt::Getopt;
    ///
    /// let args = vec!["prog".to_string(), "-a".to_string()];
    /// let parser = Getopt::new("a", args);
    /// ```
    pub fn new(optstring: &str, args: Vec<String>) -> Self {
        Self {
            args,
            optstring: optstring.to_string(),
            optind: 1,
            optopt: '?',
            optarg: None,
            opterr: true,
            nextchar_idx: 0,
        }
    }

    /// Returns the next option character, or `None` if parsing is complete.
    ///
    /// This function mimics the behavior of the standard `getopt`:
    ///
    /// - Returns `Some(char)` for a valid option
    /// - Returns `Some('?')` for an unknown option
    /// - Returns `Some(':')` if an argument is missing and `optstring` starts with `:`
    /// - Returns `None` when no more options are available
    ///
    /// # Side Effects
    ///
    /// - Updates `optind`, `optarg`, and `optopt`
    /// - May print errors to stderr if `opterr` is `true`
    ///
    /// # Rules
    ///
    /// - Options can be grouped: `-abc`
    /// - Options with arguments:
    ///   - `-o value`
    ///   - `-ovalue`
    /// - `--` stops option parsing
    ///
    /// # Example
    ///
    /// ```rust
    /// use bsd_getopt::Getopt;
    ///
    /// let args = vec![
    ///     "prog".to_string(),
    ///     "-a".to_string(),
    ///     "-b".to_string(),
    ///     "foo".to_string(),
    /// ];
    ///
    /// let mut g = Getopt::new("ab:", args);
    ///
    /// assert_eq!(g.next(), Some('a'));
    /// assert_eq!(g.next(), Some('b'));
    /// assert_eq!(g.optarg, Some("foo".to_string()));
    /// assert_eq!(g.next(), None);
    /// ```
    pub fn next(&mut self) -> Option<char> {
        self.optarg = None;

        if self.optind >= self.args.len() {
            return None;
        }

        let arg = &self.args[self.optind];

        if self.nextchar_idx == 0 {
            if !arg.starts_with('-') || arg == "-" {
                return None;
            }
            if arg == "--" {
                self.optind += 1;
                return None;
            }
            self.nextchar_idx = 1; 
        }

        let c = arg.chars().nth(self.nextchar_idx).unwrap();
        self.optopt = c;
        self.nextchar_idx += 1;

        match self.optstring.find(c) {
            None => {
                if self.nextchar_idx >= arg.len() {
                    self.optind += 1;
                    self.nextchar_idx = 0;
                }
                if self.opterr && !self.optstring.starts_with(':') {
                    eprintln!("{}: illegal option -- {}", self.args[0], c);
                }
                Some('?')
            }
            Some(idx) => {
                if self.optstring.as_bytes().get(idx + 1) == Some(&b':') {
                    if self.nextchar_idx < arg.len() {
                        self.optarg = Some(arg[self.nextchar_idx..].to_string());
                        self.optind += 1;
                        self.nextchar_idx = 0;
                    } else {
                        self.optind += 1;
                        if self.optind < self.args.len() {
                            self.optarg = Some(self.args[self.optind].clone());
                            self.optind += 1;
                        } else {
                            // 缺少参数
                            self.nextchar_idx = 0;
                            if self.optstring.starts_with(':') {
                                return Some(':');
                            } else {
                                if self.opterr {
                                    eprintln!("{}: option requires an argument -- {}", self.args[0], c);
                                }
                                return Some('?');
                            }
                        }
                    }
                    self.nextchar_idx = 0;
                } else {
                    if self.nextchar_idx >= arg.len() {
                        self.optind += 1;
                        self.nextchar_idx = 0;
                    }
                }
                Some(c)
            }
        }
    }
}

#[cfg(test)]
mod tests;