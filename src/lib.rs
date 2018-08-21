use std::collections::HashMap;

pub fn check(input: String) -> bool {
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

fn compare(prev: char, current: char) -> bool {
    let mut closer = HashMap::new();

    // HashMap to define closing char according to opened one
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
