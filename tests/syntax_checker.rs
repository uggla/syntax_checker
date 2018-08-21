extern crate syntax_checker;
use syntax_checker::check;

#[test]
// Check various combination
fn check_input() {
    assert_eq!(check("{}"), true);
    assert_eq!(check("()"), true);
    assert_eq!(check("[]"), true);
    assert_eq!(check("{[]}"), true);
    assert_eq!(check("{[]"), false);
    assert_eq!(check("[}]"), false);
    assert_eq!(check("([}])"), false);
    assert_eq!(check("("), false);
    assert_eq!(check("()}"), false);
}
