fn main() {
    my::indirect_call();
}

mod my {
    mod cool {
        pub fn function() {
            println!("called my::cool::function");
        }
    }

    fn function() {
        println!("called my::function");
    }

    pub fn indirect_call() {
        println!("called my::indirect_call that\n");

        self::function();
        function();

        self::cool::function();
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn function() {
    println!("called function");
}

mod cool {
    pub fn function() {
        println!("called cool::function");
    }
}
