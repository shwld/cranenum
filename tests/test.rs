use cranenum::Cranenum;

#[derive(Cranenum)]
// #[self_name(lowercase)]
enum RootError {
  CustomError(CustomError),
}

enum CustomError {
  MyError1(String),
  MyError2(String),
}

#[test]
fn test_can_convert() {
    let my_error1 = CustomError::MyError1("hoge".to_string());
    let _to: RootError = RootError::from(my_error1);

    let my_error2 = CustomError::MyError2("fuga".to_string());
    let _to: RootError = RootError::from(my_error2);
}
