extern crate syntax_checker;
use syntax_checker::check;

#[test]
/// Check various combination
fn check_input() {
    assert_eq!(check("{}".to_string()), true);
    assert_eq!(check("()".to_string()), true);
    assert_eq!(check("[]".to_string()), true);
    assert_eq!(check("{[]}".to_string()), true);
    assert_eq!(check("{[]".to_string()), false);
    assert_eq!(check("[}]".to_string()), false);
    assert_eq!(check("([}])".to_string()), false);
    assert_eq!(check("(".to_string()), false);
}
