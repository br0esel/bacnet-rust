use std::time::Duration;

use bacnet::{
    bip_receive, handler_read_property,
    BACnet_Confirmed_Service_Choice_SERVICE_CONFIRMED_READ_PROPERTY, Send_Read_Property_Request, MAX_MPDU, BACnet_Device_Address, address_bind_request, Send_WhoIs_To_Network,
};
use bacnet_stack_wrapper as bacnet;
use futures::executor::block_on;

fn main() {
    println!("Hello, world2!");
    // unsafe {
    //     // let te = bacnet::object_functions_t {};
    //     // let mut raw = std::ptr::null_mut();
    //     bacnet::Device_Init(std::ptr::null_mut());
    //     bacnet::apdu_set_confirmed_handler(
    //         BACnet_Confirmed_Service_Choice_SERVICE_CONFIRMED_READ_PROPERTY,
    //         Some(handler_read_property),
    //     );
    // }
    println!("This is the end2.");

    block_on(bacnet_test());
}

async fn bacnet_test(){
    println!("Begin bacnet_test.");
    std::env::set_var("BACNET_IP_DEBUG", "");

    let mut bacnet_address = {
        bacnet::BACnet_Device_Address {
            mac_len: 0,
            mac: [0;7],
            net: 0,
            len: 0,
            adr: [0;7],
        }
    };
    let bacnet_address_ptr = &mut bacnet_address as *mut BACnet_Device_Address;
    let mut device_bound = false;
    let mut device_max_mpdu = 0u32;
    let device_max_mpdu_ptr = &mut device_max_mpdu as *mut std::os::raw::c_uint;

    unsafe{
        bacnet::Device_Init(std::ptr::null_mut());
        bacnet::address_init();
        bacnet::dlenv_init();

        for x in 1..3  {
            println!("Sending whois .. {x}");
            Send_WhoIs_To_Network(bacnet_address_ptr, -1, -1);
            async_std::task::sleep(Duration::from_secs(1)).await;
        }
        device_bound = address_bind_request(12, device_max_mpdu_ptr, bacnet_address_ptr);

    }
}

async fn somehting() {
    println!("Hello from async function.");

    for x in 1..10 {
        // async_std::task::sleep(Duration::from_secs(1)).await;

        asd(x.to_string()).await;

        async_std::task::yield_now().await;
    }
    
    let mut device_bound = false;
    let mut device_max_mpdu = 0u32;
    let mut bacnet_address = {
        bacnet::BACnet_Device_Address {
            mac_len: 0,
            mac: [0;7],
            net: 0,
            len: 0,
            adr: [0;7],
        }
    };

    unsafe {
        bacnet::Device_Init(std::ptr::null_mut());
        let device_max_mpdu_ptr = &mut device_max_mpdu as *mut std::os::raw::c_uint;

        println!("-----------------");
        println!("device_max_mpdu:{}",*device_max_mpdu_ptr);

        let bacnet_address_ptr = &mut bacnet_address as *mut BACnet_Device_Address;

        while !device_bound {
           device_bound = address_bind_request(12, device_max_mpdu_ptr, bacnet_address_ptr);
           Send_WhoIs_To_Network(bacnet_address_ptr, 0, 12);
           async_std::task::sleep(Duration::from_secs(1)).await;
        }

        let result = Send_Read_Property_Request(12, 12, 12, 12, 12);

        let mut arr = [0u8; MAX_MPDU as usize];
        bip_receive(
            bacnet_address_ptr,
            arr.as_mut_ptr(),
            MAX_MPDU.try_into().expect("This should never fail."),
            11,
        );
        println!("sdfsdf");
    }
}

async fn asd(msg: String) {
    println!("{msg}");
}
