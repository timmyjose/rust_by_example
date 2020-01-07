mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function");
    }

    pub fn function() {
        println!("called my_mod::function");
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access which then ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function");
        }

        // functions declared using pub(in path) are only visible to the path
        // which must be an ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::public_function_in_my_mod which then ");
            public_function_in_nested();
        }

        // function declared with pub(self) are only visible in the current module, basically
        // the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested");
        }

        // functions declared with pub(super) are only visible to the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called my_mod::call_public_function_in_my_mod which ");
        nested::public_function_in_my_mod();
        println!(" and then ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes function visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function");
        }

        #[allow(dead_code)]
        // this is still private since its parent is private
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function");
        }
    }
}

fn main() {
    my_mod::function();
    my_mod::indirect_access();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
    my_mod::nested::function();
}
