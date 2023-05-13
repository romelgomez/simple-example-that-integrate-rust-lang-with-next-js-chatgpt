use wasm_bindgen::prelude::*;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn sum_two_int() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}
