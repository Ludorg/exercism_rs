pub fn brackets_are_balanced_v0(string: &str) -> bool {
    let mut brackets = 0;
    let mut braces = 0;
    let mut parentheses = 0;
    for c in string.chars() {
        match c {
            '[' => brackets += 1,
            ']' => brackets -= 1,
            '{' => braces += 1,
            '}' => braces -= 1,
            '(' => parentheses += 1,
            ')' => parentheses -= 1,
            _ => (),
        }
        if brackets < 0 || braces < 0 || parentheses < 0 {
            return false;
        }
        println!("{}/{}/{}", brackets, braces, parentheses);
    }
    brackets == 0 && braces == 0 && parentheses == 0
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '[' | ']' | '{' | '}' | '(' | ')' => (),
            _ => continue,
        }

        if stack.is_empty() {
            stack.push(c);
        } else {
            let top = *stack.last().unwrap();
            if (top == '{' && c == '}') || (top == '[' && c == ']') || (top == '(' && c == ')') {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }
    stack.is_empty()
}

fn main() {
    // println!("{}", brackets_are_balanced("[]"));
    // println!("{}", brackets_are_balanced("[["));
    println!("{}", brackets_are_balanced("[({]})"));
    println!("{}", brackets_are_balanced("]["));
    //println!("{}", brackets_are_balanced("([{}({}[])])"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }

    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }

    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }

    #[test]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }

    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }

    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }

    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }

    #[test]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }

    #[test]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }

    #[test]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }

    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }

    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }

    #[test]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }

    #[test]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }

    #[test]
    fn early_incomplete_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }

    #[test]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }

    #[test]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }

    #[test]
    fn complex_latex_expression() {
        let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
        assert!(brackets_are_balanced(input));
    }
}
