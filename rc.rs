use std::rc::Rc;

fn main() {
    let rc_example = "Rc String".to_string();

    {
        let rc_a = Rc::new(rc_example);
        println!("Reference count = {}", Rc::strong_count(&rc_a));

        {
            let rc_b = Rc::clone(&rc_a);
            println!("Reference count = {}", Rc::strong_count(&rc_a));
            println!("Reference count = {}", Rc::strong_count(&rc_b));

            {
                let rc_c = rc_a.clone();
                println!("Reference count = {}", Rc::strong_count(&rc_a));
                println!("Reference count = {}", Rc::strong_count(&rc_b));
                println!("Reference count = {}", Rc::strong_count(&rc_c));
            }

            println!("Reference count = {}", Rc::strong_count(&rc_b));
        }

        println!("Reference count = {}", Rc::strong_count(&rc_a));
    }
}
