pub mod boss {
    pub struct Boss {
        name: String,
        age: u8,
    }

    impl Boss {
        pub fn new(name: String, age: u8) -> Boss {
            Boss { name, age }
        }
    }
}
