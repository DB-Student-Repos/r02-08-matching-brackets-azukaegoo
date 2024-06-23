pub fn brackets_are_balanced(string: &str) -> bool {
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");
    let mut list = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => list.push(c),
            ']' => {
                if list.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if list.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if list.pop() != Some('(') {
                    return false;
                }
            }
            _ => {}
        }
    }

    list.is_empty()
}
