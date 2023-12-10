pub mod first;
pub mod second;

pub fn run(path: &str) {
    assert_eq!(2, first::execute(format!("{path}/sample").as_str()));
    assert_eq!(6, first::execute(format!("{path}/sample2").as_str()));
    assert_eq!(16697, first::execute(format!("{path}/input").as_str()));

    // assert_eq!(5905, second::execute(format!("{path}/sample").as_str()));
    // assert_eq!(246285222, second::execute(format!("{path}/input").as_str()));
    ()
}
