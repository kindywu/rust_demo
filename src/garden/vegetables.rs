pub fn vegetable() {
    println!("I'am vegetables")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vegetable() {
        vegetable()
    }
}
