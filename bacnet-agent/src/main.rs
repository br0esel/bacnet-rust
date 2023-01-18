use  bacnet_stack_wrapper as bacnet;

fn main() {
    println!("Hello, world!");
    unsafe {
        // let te = bacnet::object_functions_t {};
        // let mut raw = std::ptr::null_mut();
         bacnet::Device_Init(std::ptr::null_mut());
    }
}
