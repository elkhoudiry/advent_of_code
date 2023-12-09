pub mod first;
pub mod second;

pub fn run(path: &str) {
    assert_eq!(288, first::execute(format!("{path}/sample").as_str()));
    assert_eq!(1710720, first::execute(format!("{path}/input").as_str()));

    assert_eq!(71503, second::execute(format!("{path}/sample").as_str()));
    assert_eq!(35349468, second::execute(format!("{path}/input").as_str()));
    ()
}
