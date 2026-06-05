struct SecretData {
    value: u64,
}

fn main() {
    let my_number: i32 = 500;
    let my_secret_data = SecretData { value: 123456789 };
    let heap_number = Box::new(my_number);
    let heap_secret_data = Box::new(my_secret_data);
    println!("Heap allocated number: {}", heap_number);
    println!("Heap allocated secret data value: {}", heap_secret_data.value);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_allocation() {
        let my_number: i32 = 500;
        let my_secret_data = SecretData { value: 123456789 };
        let heap_number = Box::new(my_number);
        let heap_secret_data = Box::new(my_secret_data);

        assert_eq!(*heap_number, 500);
        assert_eq!(heap_secret_data.value, 123456789);
    }
}