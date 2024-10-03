pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' => match stack.pop() {
                Some('{') => continue,
                _ => return false,
            },
            ')' => match stack.pop() {
                Some('(') => continue,
                _ => return false,
            },
            ']' => match stack.pop() {
                Some('[') => continue,
                _ => return false,
            },
            _ => continue,
        }
    }
    stack.is_empty()
}
