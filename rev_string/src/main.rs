use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", reverse("Hello, world!"));
}

pub fn reverse(input: &str) -> String {
    //let g = str.graphemes(true);
    // UnicodeSegmentation::graphemes(input.chars().collect(), true).rev()

    // let gr1 = UnicodeSegmentation::graphemes(input, true)
    //     .rev()
    //     .collect::<String>();
    // // gr1.reverse();
    // //input.chars().rev().collect::<String>()
    // gr1

    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grapheme_clusters() {
        let input = "auüu";

        let output = reverse(input);

        let expected = "uüua";

        assert_eq!(output, expected);
    }
}
