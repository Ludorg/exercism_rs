pub fn verse(n: u32) -> String {
    let n_1 = if n != 0 { n - 1} else { n };
    match n {        
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {n_1} bottles of beer on the wall.\n")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret = String::from("");
    for n in (end..start+1).rev() {
        ret.push_str(verse(n).as_str());
        if n != end         {
            ret.push('\n');
        }
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verse_0() {
        assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    #[test]
    fn verse_1() {
        assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }

    #[test]
    fn verse_2() {
        assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }

    #[test]
    fn verse_8() {
        assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
    }

    #[test]
    fn song_8_6() {
        assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
    }

    #[test]
    fn song_3_0() {
        assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
}
fn main() {
    println!("Hello, world!");
}
