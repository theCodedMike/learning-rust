use validator::HasLen;
///
/// cargo r --bin us
///
fn main() {
    let s1 = "xyz";
    println!("   len: {}", s1.len()); //    3
    println!("length: {}", s1.length()); // 3

    let s2 = "我爱你";
    println!("   len: {}", s2.len()); //    9
    println!("length: {}", s2.length()); // 3

    let s3 = "uüu"; // "uu\314\210u"  "uu\u{308}u"
    println!("   len: {}", s3.len()); //    5
    println!("length: {}", s3.length()); // 4

    let s4 = "🚀"; // "🌑"
    println!("   len: {}", s4.len()); //    4
    println!("length: {}", s4.length()); // 1

    let s4 = "The 🚀 goes to the 🌑!";
    let rocket = utf8_slice::slice(s4, 4, 5);
    println!("{}", rocket); // 🚀

    let rocket_goes_to_the_moon = utf8_slice::from(s4, 4);
    println!("{}", rocket_goes_to_the_moon); // 🚀 goes to the 🌑!

    let the_rocket = utf8_slice::till(s4, 5);
    println!("{}", the_rocket); // The 🚀

    let len = utf8_slice::len(s3);
    println!("{}", len); // 4
}
