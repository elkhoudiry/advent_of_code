pub mod first;
pub mod second;

pub fn run(path: &str) {
    assert_eq!(6440, first::execute(format!("{path}/sample").as_str()));
    assert_eq!(248113761, first::execute(format!("{path}/input").as_str()));

    assert_eq!(5905, second::execute(format!("{path}/sample").as_str()));
    assert_eq!(246285222, second::execute(format!("{path}/input").as_str()));
    ()
}
