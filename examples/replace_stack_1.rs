extern crate psm;
use std::alloc;

const STACK_SIZE: usize = 4096 * 64;
const STACK_ALIGN: usize = 4096;

fn main() {
    println!("current stack pointer is {:p}", psm::stack_pointer());
    unsafe {
        let new_stack = alloc::alloc(alloc::Layout::from_size_align(STACK_SIZE, STACK_ALIGN).unwrap());
        println!("new stack is between {:p} and {:p}", new_stack, new_stack.offset(STACK_SIZE as isize));
        psm::replace_stack(new_stack, STACK_SIZE, || {
            println!("after replacement stack pointer is {:p}", psm::stack_pointer());
            ::std::process::exit(0);
        });
    }
}
