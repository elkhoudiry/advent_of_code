pub mod first;
pub mod second;

pub fn run(path: &str) {
    assert_eq!(114, first::execute(format!("{path}/sample").as_str()));
    assert_eq!(-114, first::execute(format!("{path}/sample2").as_str()));
    assert_eq!(1757008019, first::execute(format!("{path}/input").as_str()));

    assert_eq!(2, second::execute(format!("{path}/sample").as_str()));
    assert_eq!(995, second::execute(format!("{path}/input").as_str()));

    ()
}
