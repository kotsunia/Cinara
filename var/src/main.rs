fn main() {
    // mut = mutable ie can be allowed to change
    let mut x = 5;
    println!("number x is {x}");
    x = 6;
    println!("number x is {x}");

    const y: u32 = 60 * 60 * 3;
    println!("number y is {y}");

    // will read the first then add the second but get shadowed in the third
    let b = 5;

    let b = b + 1;
    {
        let b = b * 2;
        println!("number b is {b}");
    }

    println!("number b is {b}");

    // shadowing allows to reuse the same name for different stuff other than mut
    let spaces = "          ";
    let spaces = spaces.len();

    println!("number spaces is {spaces}");
}
