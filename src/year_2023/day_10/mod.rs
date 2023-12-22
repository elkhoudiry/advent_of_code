pub mod first;
pub mod second;

pub fn run(path: &str) {
    // assert_eq!(4, first::execute(format!("{path}/sample").as_str()));
    // assert_eq!(8, first::execute(format!("{path}/sample2").as_str()));
    // assert_eq!(6927, first::execute(format!("{path}/input").as_str()));

    assert_eq!(4, second::execute(format!("{path}/sample3").as_str()));
    assert_eq!(8, second::execute(format!("{path}/sample4").as_str()));
    assert_eq!(10, second::execute(format!("{path}/sample5").as_str()));
    assert_eq!(467, second::execute(format!("{path}/input").as_str()));

    ()
}
