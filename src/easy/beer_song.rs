pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        );
    }

    if n == 1 {
        return String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take it down and pass it around, no more bottles of beer on the wall.\n",
        );
    }

    if n == 2 {
        return String::from(
            "2 bottles of beer on the wall, 2 bottles of beer.\n\
        Take one down and pass it around, 1 bottle of beer on the wall.\n",
        );
    }

    return format!(
        "{0} bottles of beer on the wall, {0} bottles of beer.\n\
    Take one down and pass it around, {1} bottles of beer on the wall.\n",
        n,
        n - 1
    );
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    for i in (end..=start).rev() {
        result.push_str(&verse(i));
        if i != end {
            result.push_str("\n");
        }
    }
    return result;
}
