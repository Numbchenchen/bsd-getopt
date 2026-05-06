#[test]
fn test() {
    use super::*;
    let mut getopt = Getopt::new("a:b", ["./prog", "-aHello", "-b", "YES"].iter().map(|s| s.to_string()).collect());

    assert_eq!(getopt.next(), Some('a'));
    assert_eq!(getopt.optarg, Some("Hello".to_string()));
    assert_eq!(getopt.next(), Some('b'));
    assert_eq!(getopt.next(), None);


    let mut getopt = Getopt::new("fa:b:", ["prog", "-f", "-a"].iter().map(|s| s.to_string()).collect());

    getopt.opterr = false;

    assert_eq!(getopt.next(), Some('f'));
    assert_eq!(getopt.next(), Some('?'));
    
    let mut getopt = Getopt::new("fa:b:", ["prog", "-f", "-G"].iter().map(|s| s.to_string()).collect());

    getopt.opterr = false;

    assert_eq!(getopt.next(), Some('f'));
    assert_eq!(getopt.next(), Some('?'));

}