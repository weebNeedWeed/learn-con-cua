fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let str1 = "hello1";
    let str2 = "hello12";

    let str3 = longest_string(str1, str2);

    println!("{str3}");
}
