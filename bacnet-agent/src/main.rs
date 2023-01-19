use bacnet::{BACnet_Confirmed_Service_Choice_SERVICE_CONFIRMED_READ_PROPERTY, handler_read_property};
use  bacnet_stack_wrapper as bacnet;

fn main() {
    println!("Hello, world2!");
    unsafe {
        // let te = bacnet::object_functions_t {};
        // let mut raw = std::ptr::null_mut();
         bacnet::Device_Init(std::ptr::null_mut());
         bacnet::apdu_set_confirmed_handler(
            BACnet_Confirmed_Service_Choice_SERVICE_CONFIRMED_READ_PROPERTY, 
            Some(handler_read_property));
    }
    println!("This is the end2.");
}
