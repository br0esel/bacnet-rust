use std::time::{Duration, Instant};

use bacnet::{
    address_bind_request, bip_receive, npdu_handler, BACnet_Device_Address,
    Send_Read_Property_Request, Send_WhoIs_To_Network, MAX_MPDU, Send_WhoIs,
};
use bacnet_stack_wrapper as bacnet;
use futures::executor::block_on;

fn main() {
    std::env::set_var("BACNET_IP_DEBUG", "");

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
    unsafe {
        bacnet::Device_Init(std::ptr::null_mut());
        bacnet::address_init();
        bacnet::dlenv_init();

        bacnet::apdu_set_unconfirmed_handler(bacnet::BACnet_Unconfirmed_Service_Choice_SERVICE_UNCONFIRMED_WHO_IS, Some(bacnet::handler_who_is));
        bacnet::apdu_set_unconfirmed_handler(bacnet::BACnet_Unconfirmed_Service_Choice_SERVICE_UNCONFIRMED_I_AM, Some(bacnet::handler_i_am_add));
    }
    block_on(bacnet_loop());
    let result = unsafe {Send_Read_Property_Request(1234, 12, 12, 12, 12);};
    println!("ende");
}

async fn bacnet_loop() {

    let mut device_max_mpdu = 0u32;
    let device_max_mpdu_ptr = &mut device_max_mpdu as *mut std::os::raw::c_uint;
    let mut bacnet_address = {
        bacnet::BACnet_Device_Address {
            mac_len: 0,
            mac: [0; 7],
            net: 0,
            len: 0,
            adr: [0; 7],
        }
    };
    let bacnet_address_ptr = &mut bacnet_address as *mut BACnet_Device_Address;
    for x in 1..10 {
        // bacnet_test().await;
        let mut device_bound =
            unsafe { address_bind_request(1234, device_max_mpdu_ptr, bacnet_address_ptr) };
        unsafe {
            Send_WhoIs_To_Network(bacnet_address_ptr, -1, -1);
            Send_WhoIs(1234,1234);
        }
        async_std::task::sleep(Duration::from_millis(500)).await;
        println!("1device_bound:{}", device_bound);
        bacnet_data_receive_handler();
        println!("d2evice_bound:{}", device_bound);
        async_std::task::sleep(Duration::from_millis(500)).await;
    }
}

fn bacnet_data_receive_handler() {
    let mut device_bound = false;
    let mut device_max_mpdu = 0u32;
    let mut arr = [0u8; MAX_MPDU as usize];

    let mut bacnet_address = {
        bacnet::BACnet_Device_Address {
            mac_len: 0,
            mac: [0; 7],
            net: 0,
            len: 0,
            adr: [0; 7],
        }
    };

    let bacnet_address_ptr = &mut bacnet_address as *mut BACnet_Device_Address;
    let pdu_len: u16;
    unsafe {
        pdu_len = bip_receive(
            bacnet_address_ptr,
            arr.as_mut_ptr(),
            MAX_MPDU.try_into().expect("This should never fail."),
            100,
        );
        let start = Instant::now();
        if pdu_len > 0 {
            npdu_handler(bacnet_address_ptr, arr.as_mut_ptr(), pdu_len);
            // npdu_handler(src, pdu, pdu_len)
            // npdu_handler(&src, &Rx_Buf[0], pdu_len);
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?} with {}", duration, pdu_len);
    }
    println!("End of receive handler.");
}

async fn bacnet_test() {
    println!("Begin bacnet_test.");
    std::env::set_var("BACNET_IP_DEBUG", "");

    let mut bacnet_address = {
        bacnet::BACnet_Device_Address {
            mac_len: 0,
            mac: [0; 7],
            net: 0,
            len: 0,
            adr: [0; 7],
        }
    };
    let bacnet_address_ptr = &mut bacnet_address as *mut BACnet_Device_Address;
    let mut device_bound = false;
    let mut device_max_mpdu = 0u32;
    let device_max_mpdu_ptr = &mut device_max_mpdu as *mut std::os::raw::c_uint;

    unsafe {
        bacnet::Device_Init(std::ptr::null_mut());
        bacnet::address_init();
        bacnet::dlenv_init();

        for x in 1..2 {
            println!("Sending whois .. {x}");
            Send_WhoIs_To_Network(bacnet_address_ptr, -1, -1);
            async_std::task::sleep(Duration::from_secs(1)).await;
        }
        device_bound = address_bind_request(1234, device_max_mpdu_ptr, bacnet_address_ptr);

        let result = Send_Read_Property_Request(1234, 12, 12, 12, 12);

        println!("bacnet_test end.")
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
            mac: [0; 7],
            net: 0,
            len: 0,
            adr: [0; 7],
        }
    };

    unsafe {
        bacnet::Device_Init(std::ptr::null_mut());
        let device_max_mpdu_ptr = &mut device_max_mpdu as *mut std::os::raw::c_uint;

        println!("-----------------");
        println!("device_max_mpdu:{}", *device_max_mpdu_ptr);

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
