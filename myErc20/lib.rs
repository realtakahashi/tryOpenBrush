#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// use ink_lang as ink;

#[brush::contract]
mod myErc20 {
//    use brush::contracts::psp22::*;
    use brush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct MyErc20 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for MyErc20 {}
    impl PSP22Metadata for MyErc20 {}

    impl MyErc20{
        // #[ink(constructor)]
        // pub fn new(total_supply: Balance) -> Self {
        //     ink_lang::codegen::initialize_contract(|instance: &mut MyErc20| {
        //         instance
        //             ._mint(instance.env().caller(), total_supply)
        //             .expect("Should mint");
        //     })
        // }
        
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = decimal;
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint total_supply");
            })
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let myErc20 = MyErc20::default();
            assert_eq!(myErc20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut myErc20 = MyErc20::new(false);
            assert_eq!(myErc20.get(), false);
            myErc20.flip();
            assert_eq!(myErc20.get(), true);
        }
    }
}
