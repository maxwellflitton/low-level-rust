

#[cfg(test)]
mod tests {
    use super::*;
    use base_layer::collections::vector::SimpleVec;

    #[test]
    fn test_simple_vec() {
        let mut vec = SimpleVec::new(4);

        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(vec.length, 4);

        let mut counter = 0;

        for i in vec {
            counter += 1;
            assert_eq!(i, counter);
        }
    }
}
