use std::collections::HashMap;

/// Compare previous char and current one
fn compare(prev: char, current: char) -> bool {
    // HashMap to define closing char according to opened one
    let mut closer = HashMap::new();
    closer.insert('{', '}');
    closer.insert('(', ')');
    closer.insert('[', ']');

    match closer.get(&prev) {
        Some(x) => *x == current,
        None => false,
    }
}

/// Check if input with open brackets or parenthesis are respectively closed
///
/// # Examples :
///
/// ```
/// assert_eq!(syntax_checker::check("{[]}"), true);
/// ```
pub fn check(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for i in input.chars() {
        match i {
            '[' | '(' | '{' => stack.push(i),
            ']' | ')' | '}' => match stack.pop() {
                Some(prev) => {
                    if !compare(prev, i) {
                        return false;
                    }
                }
                None => return false,
            },
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_test() {
        assert_eq!(compare('{', '}'), true);
        assert_eq!(compare('(', ')'), true);
        assert_eq!(compare('[', ']'), true);
        assert_eq!(compare('{', ']'), false);
        assert_eq!(compare('(', ']'), false);
    }

    #[test]
    // Check various combination
    fn check_test() {
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
}
