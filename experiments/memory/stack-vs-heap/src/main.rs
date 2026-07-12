fn main() {
    let stack_value = 42_u64;
    let boxed_value = Box::new(84_u64);
    let stack_reference = &stack_value;
    let boxed_reference = boxed_value.as_ref();

    println!("stack value:           {}", stack_value);
    println!("stack value via ref:   {}", *stack_reference);
    println!("boxed value via ref:   {}", *boxed_reference);

    println!("stack value address:   {:p}", stack_reference);
    println!("Box handle address:    {:p}", &boxed_value);
    println!("boxed value address:   {:p}", boxed_reference);
}
