#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

macro_rules! color {
    (reset) => {"\x1b[0m"};
    (red) => {"\x1b[31m"};
    (green) => {"\x1b[32m"};
    (yellow) => {"\x1b[33m"};
    (blue) => {"\x1b[34m"};
    (magenta) => {"\x1b[35m"};
    (cyan) => {"\x1b[36m"};
    (white) => {"\x1b[37m"};
    (bold) => {"\x1b[1m"};
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("{}[WithColor]: Hello, Arceos!",color!(green));
    // println!(
    //     "{}[WithColor]:{} {}Hello, {}ArceOS{}!",
    //     color!(bold),
    //     color!(reset),
    //     color!(green),
    //     color!(cyan),
    //     color!(reset)
    // );    
    // println!(
    //     "{}Error:{} {}Something went wrong{}",
    //     color!(red),
    //     color!(reset),
    //     color!(yellow),
    //     color!(reset)
    // );
}
