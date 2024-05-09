pub mod vegetables;

pub fn garden() {
    println!("I'am garden")
}

#[cfg(test)]
mod tests {
    use super::garden;

    #[test]
    fn test_garden() {
        garden();
    }
}
