fn main() {
    #[cfg(feature = "debug")]
    println!("Debug mode is enabled!");

    #[cfg(feature = "release")]
    println!("Release mode is enabled!");

    println!("Hello, world!");
}
