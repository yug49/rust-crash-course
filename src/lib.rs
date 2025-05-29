use cfg_if::cfg_if;

#[cfg(feature = "exercises")]
pub mod exercises {
    pub fn foo() -> u32 {
        println!("Feature 'ex' is enabled!");
        0
    }
}

#[cfg(feature = "solutions")]
pub mod solutions {
    pub fn foo() -> u32 {
        println!("Feature 'sol' is enabled!");
        1
    }
}

cfg_if! {
    if #[cfg(feature = "solutions")] {
        pub use solutions as target;
    } else {
        pub use exercises as target;
    }
}
