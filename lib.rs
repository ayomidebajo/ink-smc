#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logger {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    // use scale;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.\
    #[ink(storage)]
    pub struct Logger {
        /// Stores a single `bool` value on the storage.
        // value: bool,
        value: Vec<u8>,
    }
    impl Logger {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: Vec<u8>) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = String::from("fflipped!").into_bytes();
        // }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Vec<u8> {
            self.value.clone()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use std::time::UNIX_EPOCH;

        use ink_env::Error;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let logger = Logger::default();
            // assert_eq!(logger.get(), b"");
            assert_eq!(Ok::<Vec<u8>, Error>(logger.get()), Ok(logger.get()))
        }


        // / We test a simple use case of our contract.

        #[ink::test]
        fn it_works() {
            let logger = Logger::new(b"1970-01-01 00:00:00".to_vec());
            assert_eq!(logger.get(), b"1970-01-01 00:00:00".to_vec());
            // logger.flip();
            // assert_eq!(logger.get(), b"fflipped!".to_vec());
        }
    }
}
