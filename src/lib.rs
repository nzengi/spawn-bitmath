use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

/// Serde ile `u128` serileştirilmesi ve deseralize edilmesi için yapı.
/// WebAssembly'de `u128`'i işler hale getirmek için kullanılır.
#[derive(Serialize, Deserialize)]
pub struct U128Wrapper {
    value: u128,
}

/// Computes the index of the most significant bit of the u128 number.
/// The least significant bit is at index 0, and the most significant bit is at index 127.
///
/// # Arguments
/// * `input` - Serde ile serileştirilen u128 değeri.
///
/// # Returns
/// The index of the most significant bit.
#[wasm_bindgen]
pub fn most_significant_bit(input: &JsValue) -> Result<JsValue, JsValue> {
    let wrapper: U128Wrapper = from_value(input.clone()).map_err(|e| JsValue::from_str(&format!("Invalid input: {}", e)))?;
    let value = wrapper.value;

    if value == 0 {
        return Err(JsValue::from_str("Input must be greater than 0"));
    }

    let mut x = value;
    let mut r: u8 = 0;

    if x >= 0x10000000000000000 {
        x >>= 64;
        r += 64;
    }
    if x >= 0x100000000 {
        x >>= 32;
        r += 32;
    }
    if x >= 0x10000 {
        x >>= 16;
        r += 16;
    }
    if x >= 0x100 {
        x >>= 8;
        r += 8;
    }
    if x >= 0x10 {
        x >>= 4;
        r += 4;
    }
    if x >= 0x4 {
        x >>= 2;
        r += 2;
    }
    if x >= 0x2 {
        r += 1;
    }

    to_value(&r).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Computes the index of the least significant bit of the u128 number.
/// The least significant bit is at index 0, and the most significant bit is at index 127.
///
/// # Arguments
/// * `input` - Serde ile serileştirilen u128 değeri.
///
/// # Returns
/// The index of the least significant bit.
#[wasm_bindgen]
pub fn least_significant_bit(input: &JsValue) -> Result<JsValue, JsValue> {
    let wrapper: U128Wrapper = from_value(input.clone()).map_err(|e| JsValue::from_str(&format!("Invalid input: {}", e)))?;
    let value = wrapper.value;

    if value == 0 {
        return Err(JsValue::from_str("Input must be greater than 0"));
    }

    let mut x = value;
    let mut r: u8 = 127;

    if x & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF > 0 {
        r -= 64;
    } else {
        x >>= 64;
    }
    if x & 0xFFFFFFFF > 0 {
        r -= 32;
    } else {
        x >>= 32;
    }
    if x & 0xFFFF > 0 {
        r -= 16;
    } else {
        x >>= 16;
    }
    if x & 0xFF > 0 {
        r -= 8;
    } else {
        x >>= 8;
    }
    if x & 0xF > 0 {
        r -= 4;
    } else {
        x >>= 4;
    }
    if x & 0x3 > 0 {
        r -= 2;
    } else {
        x >>= 2;
    }
    if x & 0x1 > 0 {
        r -= 1;
    }

    to_value(&r).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::to_value;

    #[test]
    fn test_most_significant_bit() {
        let input = U128Wrapper { value: 128 };
        let input_js = to_value(&input).unwrap();
        assert_eq!(most_significant_bit(&input_js).unwrap(), JsValue::from(7));
    }

    #[test]
    fn test_least_significant_bit() {
        let input = U128Wrapper { value: 16 };
        let input_js = to_value(&input).unwrap();
        assert_eq!(least_significant_bit(&input_js).unwrap(), JsValue::from(4));
    }

    #[test]
    fn test_invalid_input() {
        let input = U128Wrapper { value: 0 };
        let input_js = to_value(&input).unwrap();
        assert!(most_significant_bit(&input_js).is_err());
        assert!(least_significant_bit(&input_js).is_err());
    }
}
