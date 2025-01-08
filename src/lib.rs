#![no_std]

pub const fn repellant() -> &'static str{
    return include_str!("../data.txt");
}