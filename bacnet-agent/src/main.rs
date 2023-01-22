use std::{
    ptr::read,
    time::{Duration, Instant},
};

use bacnet::{
    address_bind_request, bip_receive, npdu_handler, BACnet_Device_Address,
    Send_Read_Property_Request, Send_WhoIs, Send_WhoIs_To_Network, MAX_MPDU,
};
use bacnet_stack_wrapper as bacnet;

use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        bacnet::Device_Init(std::ptr::null_mut());
        bacnet::address_init();
        bacnet::dlenv_init();

        bacnet::apdu_set_unconfirmed_handler(
            bacnet::BACnet_Unconfirmed_Service_Choice_SERVICE_UNCONFIRMED_WHO_IS,
            Some(bacnet::handler_who_is),
        );
        bacnet::apdu_set_unconfirmed_handler(
            bacnet::BACnet_Unconfirmed_Service_Choice_SERVICE_UNCONFIRMED_I_AM,
            Some(bacnet::handler_i_am_add),
        );
    }

    // unsafe {bacnet::Send_WhoIs(1234, 1234);}

    let t1 = tokio::spawn(async move {
        println!("fut1");
    });

    let local = tokio::task::LocalSet::new();

    let t3 = local.spawn_local(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        for x in 1..10 {
            println!("t3");
            interval.tick().await;
        }
    });

    let t2 = local.spawn_local(async move {
        let mut device_max_mpdu: std::os::raw::c_uint = 0;
        let device_max_mpdu_ptr: *mut u32 = &mut device_max_mpdu;
        let mut test: Box<u32> = Box::new(5);
        let test2: *mut u32 = &mut *test;

        let max_apdu = 0u32 as *mut std::os::raw::c_uint;
        let mut bacnet_address = bacnet::BACNET_ADDRESS {
            mac_len: 0,
            mac: [0; 7],
            net: 0,
            len: 0,
            adr: [0; 7],
        };
        let et = &mut bacnet_address as *mut BACnet_Device_Address;

        unsafe {
            bacnet::address_bind_request(1234, device_max_mpdu_ptr, et);
        }

        let mut t = tokio::time::interval(Duration::from_secs(2));
        for x in 1..10 {
            unsafe {
                bacnet::Send_WhoIs(1234, 1234);
            }
            println!("fut2={}", x);
            t.tick().await;
        }
    });

    local.await;
    // tokio::time::sleep(Duration::from_secs(3)).await;
    // t2.await;
    Ok(())
}

async fn read_prop() -> Result<String> {
    unsafe {
        bacnet::Send_Read_Property_Request(1234, 1, 1, 1, 0);
    }
    Ok("sdfsdfsdfsfd".to_string())
}

async fn bacnet_data_receive_handler() {
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
    unsafe {
        let pdu_len = bip_receive(
            bacnet_address_ptr,
            arr.as_mut_ptr(),
            MAX_MPDU.try_into().unwrap(),
            100,
        );
        if pdu_len > 0 {
            bacnet::npdu_handler(bacnet_address_ptr, arr.as_mut_ptr(), pdu_len);
        }
    }
    println!("End of receive handler.");
    tokio::task::yield_now().await;
}
