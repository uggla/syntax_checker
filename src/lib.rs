use std::collections::HashMap;

/// Check if input with open brackets or parenthesis are respectively closed
pub fn check(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for i in input.chars() {
        match i {
            '[' | '(' | '{' => stack.push(i),
            ']' | ')' | '}' => match stack.pop() {
                Some(prev) => {
                    if compare(prev, i) != true {
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

/// Compare previous char and current one
fn compare(prev: char, current: char) -> bool {
    // HashMap to define closing char according to opened one
    let mut closer = HashMap::new();
    closer.insert('{', '}');
    closer.insert('(', ')');
    closer.insert('[', ']');

    match closer.get(&prev) {
        Some(x) => if *x == current {
            true
        } else {
            false
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Check various combination
    fn checks() {
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
