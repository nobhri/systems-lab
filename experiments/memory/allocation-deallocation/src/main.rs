struct TrackedValue {
    name: &'static str,
}

impl Drop for TrackedValue {
    fn drop(&mut self) {
        println!("dropping: {}", self.name);
    }
}

fn main() {
    println!("entering outer scope");

    {
        println!("entering inner scope");

        let boxed_value = Box::new(TrackedValue {
            name: "boxed value",
        });

        println!("using: {}", boxed_value.name);
        println!("leaving inner scope");
    }

    println!("back in outer scope");
}
