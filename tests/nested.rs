#[allow(dead_code)]
mod outer {
    #[allow(dead_code)]
    mod inner {
        #[allow(dead_code)]
        fn inner_function() {
            // function implementation
        }
    }

    #[deny(warnings)]
    fn outer_function() {
        // function implementation
    }
}
