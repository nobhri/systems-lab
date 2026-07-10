fn main() {
    let stack_value = 42_u64;
    let boxed_value = Box::new(84_u64);

    println!("stack value address:  {:p}", &stack_value);
    println!("Box handle address:   {:p}", &boxed_value);
    println!("boxed value address:  {:p}", boxed_value.as_ref());
}
