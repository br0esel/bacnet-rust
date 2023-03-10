use std::{path::PathBuf, env};

extern crate bindgen;

fn main() {
    // println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rerun-if-changed=wrapper.h");
     compile_bacnet_lib();
    create_bacnet_wrapper();
}

fn create_bacnet_wrapper() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // .clang_arg("-F/usr/lib/gcc/x86_64-linux-gnu/9/include")
        .clang_arg("-F../bacnet-stack/src")
        .clang_arg("-Wno-sign-compare")
        // .clang_arg("-I../bacnet-stack/src")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_file("stdlib.h")
        .blocklist_file("stdio.h")
        .blocklist_file("math.h")
        .blocklist_file("string.h")
        .blocklist_file("stddef.h")
        .blocklist_file("stdint.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_bacnet_lib() {
    

    let mut src_files = vec![
        "../bacnet-stack/src/bacnet/abort.c",
        //"../bacnet-stack/src/bacnet/abort.h",
        "../bacnet-stack/src/bacnet/access_rule.c",
        //"../bacnet-stack/src/bacnet/access_rule.h",
        "../bacnet-stack/src/bacnet/alarm_ack.c",
        //"../bacnet-stack/src/bacnet/alarm_ack.h",
        //"../bacnet-stack/src/bacnet/apdu.h",
        "../bacnet-stack/src/bacnet/arf.c",
        //"../bacnet-stack/src/bacnet/arf.h",
        "../bacnet-stack/src/bacnet/assigned_access_rights.c",
        //"../bacnet-stack/src/bacnet/assigned_access_rights.h",
        "../bacnet-stack/src/bacnet/authentication_factor.c",
        "../bacnet-stack/src/bacnet/authentication_factor_format.c",
        //"../bacnet-stack/src/bacnet/authentication_factor_format.h",
        //"../bacnet-stack/src/bacnet/authentication_factor.h",
        "../bacnet-stack/src/bacnet/awf.c",
        //"../bacnet-stack/src/bacnet/awf.h",
        "../bacnet-stack/src/bacnet/bacaddr.c",
        //"../bacnet-stack/src/bacnet/bacaddr.h",
        "../bacnet-stack/src/bacnet/bacapp.c",
        //"../bacnet-stack/src/bacnet/bacapp.h",
        "../bacnet-stack/src/bacnet/bacdcode.c",
        //"../bacnet-stack/src/bacnet/bacdcode.h",
        //"../bacnet-stack/src/bacnet/bacdef.h",
        "../bacnet-stack/src/bacnet/bacdevobjpropref.c",
        //"../bacnet-stack/src/bacnet/bacdevobjpropref.h",
        // "../bacnet-stack/src/bacnet/bacenum.h",
        "../bacnet-stack/src/bacnet/bacerror.c",
        //"../bacnet-stack/src/bacnet/bacerror.h",
        "../bacnet-stack/src/bacnet/bacint.c",
        //"../bacnet-stack/src/bacnet/bacint.h",
        "../bacnet-stack/src/bacnet/bacprop.c",
        //"../bacnet-stack/src/bacnet/bacprop.h",
        "../bacnet-stack/src/bacnet/bacpropstates.c",
        //"../bacnet-stack/src/bacnet/bacpropstates.h",
        "../bacnet-stack/src/bacnet/bacreal.c",
        //"../bacnet-stack/src/bacnet/bacreal.h",
        "../bacnet-stack/src/bacnet/bacstr.c",
        //"../bacnet-stack/src/bacnet/bacstr.h",
        "../bacnet-stack/src/bacnet/bactext.c",
        //"../bacnet-stack/src/bacnet/bactext.h",
        "../bacnet-stack/src/bacnet/bactimevalue.c",
        //"../bacnet-stack/src/bacnet/bactimevalue.h",
        "../bacnet-stack/src/bacnet/dailyschedule.c",
        //"../bacnet-stack/src/bacnet/dailyschedule.h",
        "../bacnet-stack/src/bacnet/weeklyschedule.c",
        //"../bacnet-stack/src/bacnet/weeklyschedule.h",
        "../bacnet-stack/src/bacnet/basic/bbmd/h_bbmd.c",
        //"../bacnet-stack/src/bacnet/basic/bbmd/h_bbmd.h",
        "../bacnet-stack/src/bacnet/basic/binding/address.c",
        //"../bacnet-stack/src/bacnet/basic/binding/address.h",
        "../bacnet-stack/src/bacnet/basic/npdu/h_npdu.c",
        //"../bacnet-stack/src/bacnet/basic/npdu/h_npdu.h",
        "../bacnet-stack/src/bacnet/basic/npdu/h_routed_npdu.c",
        //"../bacnet-stack/src/bacnet/basic/npdu/h_routed_npdu.h",
        "../bacnet-stack/src/bacnet/basic/npdu/s_router.c",
        //"../bacnet-stack/src/bacnet/basic/npdu/s_router.h",
        "../bacnet-stack/src/bacnet/basic/object/access_credential.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_credential.h",
        "../bacnet-stack/src/bacnet/basic/object/access_door.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_door.h",
        "../bacnet-stack/src/bacnet/basic/object/access_point.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_point.h",
        "../bacnet-stack/src/bacnet/basic/object/access_rights.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_rights.h",
        "../bacnet-stack/src/bacnet/basic/object/access_user.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_user.h",
        "../bacnet-stack/src/bacnet/basic/object/access_zone.c",
        //"../bacnet-stack/src/bacnet/basic/object/access_zone.h",
        "../bacnet-stack/src/bacnet/basic/object/acc.c",
        "../bacnet-stack/src/bacnet/basic/object/ai.c",
        //"../bacnet-stack/src/bacnet/basic/object/ai.h",
        "../bacnet-stack/src/bacnet/basic/object/ao.c",
        //"../bacnet-stack/src/bacnet/basic/object/ao.h",
        "../bacnet-stack/src/bacnet/basic/object/av.c",
        //"../bacnet-stack/src/bacnet/basic/object/av.h",
        "../bacnet-stack/src/bacnet/basic/object/bacfile.c",
        //"../bacnet-stack/src/bacnet/basic/object/bacfile.h",
        "../bacnet-stack/src/bacnet/basic/object/bi.c",
        //"../bacnet-stack/src/bacnet/basic/object/bi.h",
        "../bacnet-stack/src/bacnet/basic/object/bo.c",
        //"../bacnet-stack/src/bacnet/basic/object/bo.h",
        "../bacnet-stack/src/bacnet/basic/object/bv.c",
        //"../bacnet-stack/src/bacnet/basic/object/bv.h",
        "../bacnet-stack/src/bacnet/basic/object/channel.c",
        //"../bacnet-stack/src/bacnet/basic/object/channel.h",
        "../bacnet-stack/src/bacnet/basic/object/color_object.c",
        //"../bacnet-stack/src/bacnet/basic/object/color_object.h",
        "../bacnet-stack/src/bacnet/basic/object/color_temperature.c",
        //"../bacnet-stack/src/bacnet/basic/object/color_temperature.h",
        // "../bacnet-stack/src/bacnet/basic/object/client/device-client.c",
        "../bacnet-stack/src/bacnet/basic/object/command.c",
        //"../bacnet-stack/src/bacnet/basic/object/command.h",
        "../bacnet-stack/src/bacnet/basic/object/credential_data_input.c",
        //"../bacnet-stack/src/bacnet/basic/object/credential_data_input.h",
        "../bacnet-stack/src/bacnet/basic/object/csv.c",
        //"../bacnet-stack/src/bacnet/basic/object/csv.h",
        "../bacnet-stack/src/bacnet/basic/object/device.c",
        //"../bacnet-stack/src/bacnet/basic/object/device.h",
        "../bacnet-stack/src/bacnet/basic/object/iv.c",
        //"../bacnet-stack/src/bacnet/basic/object/iv.h",
        "../bacnet-stack/src/bacnet/basic/object/lc.c",
        //"../bacnet-stack/src/bacnet/basic/object/lc.h",
        "../bacnet-stack/src/bacnet/basic/object/lo.c",
        //"../bacnet-stack/src/bacnet/basic/object/lo.h",
        "../bacnet-stack/src/bacnet/basic/object/lsp.c",
        //"../bacnet-stack/src/bacnet/basic/object/lsp.h",
        "../bacnet-stack/src/bacnet/basic/object/ms-input.c",
        //"../bacnet-stack/src/bacnet/basic/object/ms-input.h",
        "../bacnet-stack/src/bacnet/basic/object/mso.c",
        //"../bacnet-stack/src/bacnet/basic/object/mso.h",
        "../bacnet-stack/src/bacnet/basic/object/msv.c",
        //"../bacnet-stack/src/bacnet/basic/object/msv.h",
        "../bacnet-stack/src/bacnet/basic/object/nc.c",
        //"../bacnet-stack/src/bacnet/basic/object/nc.h",
        "../bacnet-stack/src/bacnet/basic/object/netport.c",
        //"../bacnet-stack/src/bacnet/basic/object/netport.h",
        "../bacnet-stack/src/bacnet/basic/object/objects.c",
        //"../bacnet-stack/src/bacnet/basic/object/objects.h",
        "../bacnet-stack/src/bacnet/basic/object/osv.c",
        //"../bacnet-stack/src/bacnet/basic/object/osv.h",
        "../bacnet-stack/src/bacnet/basic/object/piv.c",
        //"../bacnet-stack/src/bacnet/basic/object/piv.h",
        "../bacnet-stack/src/bacnet/basic/object/schedule.c",
        //"../bacnet-stack/src/bacnet/basic/object/schedule.h",
        "../bacnet-stack/src/bacnet/basic/object/trendlog.c",
        //"../bacnet-stack/src/bacnet/basic/object/trendlog.h",
        "../bacnet-stack/src/bacnet/basic/service/h_alarm_ack.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_alarm_ack.h",
        "../bacnet-stack/src/bacnet/basic/service/h_apdu.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_apdu.h",
        "../bacnet-stack/src/bacnet/basic/service/h_arf_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_arf_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_arf.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_arf.h",
        "../bacnet-stack/src/bacnet/basic/service/h_awf.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_awf.h",
        "../bacnet-stack/src/bacnet/basic/service/h_ccov.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_ccov.h",
        "../bacnet-stack/src/bacnet/basic/service/h_cov.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_cov.h",
        "../bacnet-stack/src/bacnet/basic/service/h_dcc.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_dcc.h",
        "../bacnet-stack/src/bacnet/basic/service/h_gas_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_gas_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_get_alarm_sum.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_get_alarm_sum.h",
        "../bacnet-stack/src/bacnet/basic/service/h_getevent_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_getevent_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_getevent.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_getevent.h",
        "../bacnet-stack/src/bacnet/basic/service/h_iam.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_iam.h",
        "../bacnet-stack/src/bacnet/basic/service/h_ihave.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_ihave.h",
        "../bacnet-stack/src/bacnet/basic/service/h_lso.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_lso.h",
        "../bacnet-stack/src/bacnet/basic/service/h_noserv.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_noserv.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rd.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rd.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rp_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rp_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rp.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rp.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rpm_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rpm_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rpm.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rpm.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rr_a.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rr_a.h",
        "../bacnet-stack/src/bacnet/basic/service/h_rr.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_rr.h",
        "../bacnet-stack/src/bacnet/basic/service/h_ts.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_ts.h",
        "../bacnet-stack/src/bacnet/basic/service/h_ucov.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_ucov.h",
        "../bacnet-stack/src/bacnet/basic/service/h_upt.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_upt.h",
        "../bacnet-stack/src/bacnet/basic/service/h_whohas.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_whohas.h",
        "../bacnet-stack/src/bacnet/basic/service/h_whois.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_whois.h",
        "../bacnet-stack/src/bacnet/basic/service/h_wp.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_wp.h",
        "../bacnet-stack/src/bacnet/basic/service/h_wpm.c",
        //"../bacnet-stack/src/bacnet/basic/service/h_wpm.h",
        "../bacnet-stack/src/bacnet/basic/service/s_abort.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_abort.h",
        "../bacnet-stack/src/bacnet/basic/service/s_ack_alarm.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_ack_alarm.h",
        "../bacnet-stack/src/bacnet/basic/service/s_arfs.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_arfs.h",
        "../bacnet-stack/src/bacnet/basic/service/s_awfs.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_awfs.h",
        "../bacnet-stack/src/bacnet/basic/service/s_cevent.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_cevent.h",
        "../bacnet-stack/src/bacnet/basic/service/s_cov.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_cov.h",
        "../bacnet-stack/src/bacnet/basic/service/s_dcc.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_dcc.h",
        "../bacnet-stack/src/bacnet/basic/service/s_error.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_error.h",
        "../bacnet-stack/src/bacnet/basic/service/s_get_alarm_sum.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_get_alarm_sum.h",
        "../bacnet-stack/src/bacnet/basic/service/s_get_event.c",
        "../bacnet-stack/src/bacnet/basic/service/s_getevent.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_get_event.h",
        //"../bacnet-stack/src/bacnet/basic/service/s_getevent.h",
        "../bacnet-stack/src/bacnet/basic/service/s_iam.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_iam.h",
        "../bacnet-stack/src/bacnet/basic/service/s_ihave.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_ihave.h",
        "../bacnet-stack/src/bacnet/basic/service/s_lso.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_lso.h",
        "../bacnet-stack/src/bacnet/basic/service/s_rd.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_rd.h",
        "../bacnet-stack/src/bacnet/basic/service/s_readrange.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_readrange.h",
        "../bacnet-stack/src/bacnet/basic/service/s_rp.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_rp.h",
        "../bacnet-stack/src/bacnet/basic/service/s_rpm.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_rpm.h",
        "../bacnet-stack/src/bacnet/basic/service/s_ts.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_ts.h",
        "../bacnet-stack/src/bacnet/basic/service/s_uevent.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_uevent.h",
        "../bacnet-stack/src/bacnet/basic/service/s_upt.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_upt.h",
        "../bacnet-stack/src/bacnet/basic/service/s_whohas.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_whohas.h",
        "../bacnet-stack/src/bacnet/basic/service/s_whois.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_whois.h",
        "../bacnet-stack/src/bacnet/basic/service/s_wp.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_wp.h",
        "../bacnet-stack/src/bacnet/basic/service/s_wpm.c",
        //"../bacnet-stack/src/bacnet/basic/service/s_wpm.h",
        //"../bacnet-stack/src/bacnet/basic/services.h",
        "../bacnet-stack/src/bacnet/basic/sys/bigend.c",
        //"../bacnet-stack/src/bacnet/basic/sys/bigend.h",
        "../bacnet-stack/src/bacnet/basic/sys/color_rgb.c",
        //"../bacnet-stack/src/bacnet/basic/sys/color_rgb.h",
        "../bacnet-stack/src/bacnet/basic/sys/days.c",
        //"../bacnet-stack/src/bacnet/basic/sys/days.h",
        "../bacnet-stack/src/bacnet/basic/sys/debug.c",
        //"../bacnet-stack/src/bacnet/basic/sys/debug.h",
        "../bacnet-stack/src/bacnet/basic/sys/fifo.c",
        //"../bacnet-stack/src/bacnet/basic/sys/fifo.h",
        "../bacnet-stack/src/bacnet/basic/sys/filename.c",
        //"../bacnet-stack/src/bacnet/basic/sys/filename.h",
        //"../bacnet-stack/src/bacnet/basic/sys/key.h",
        "../bacnet-stack/src/bacnet/basic/sys/keylist.c",
        //"../bacnet-stack/src/bacnet/basic/sys/keylist.h",
        "../bacnet-stack/src/bacnet/basic/sys/mstimer.c",
        //"../bacnet-stack/src/bacnet/basic/sys/mstimer.h",
        "../bacnet-stack/src/bacnet/basic/sys/ringbuf.c",
        //"../bacnet-stack/src/bacnet/basic/sys/ringbuf.h",
        "../bacnet-stack/src/bacnet/basic/sys/sbuf.c",
        //"../bacnet-stack/src/bacnet/basic/sys/sbuf.h",
        "../bacnet-stack/src/bacnet/basic/tsm/tsm.c",
        //"../bacnet-stack/src/bacnet/basic/tsm/tsm.h",
        //"../bacnet-stack/src/bacnet/bits.h",
        //"../bacnet-stack/src/bacnet/bytes.h",
        //"../bacnet-stack/src/bacnet/config.h",
        "../bacnet-stack/src/bacnet/cov.c",
        //"../bacnet-stack/src/bacnet/cov.h",
        "../bacnet-stack/src/bacnet/credential_authentication_factor.c",
        //"../bacnet-stack/src/bacnet/credential_authentication_factor.h",
        //"../bacnet-stack/src/bacnet/datalink/arcnet.h",
        "../bacnet-stack/src/bacnet/datalink/bacsec.c",
        //"../bacnet-stack/src/bacnet/datalink/bacsec.h",
        //"../bacnet-stack/src/bacnet/datalink/bip6.h",
        //"../bacnet-stack/src/bacnet/datalink/bip.h",
        "../bacnet-stack/src/bacnet/datalink/bvlc6.c",
        //"../bacnet-stack/src/bacnet/datalink/bvlc6.h",
        //"../bacnet-stack/src/bacnet/datalink/bvlc.h",
        "../bacnet-stack/src/bacnet/datalink/bvlc.c",
        "../bacnet-stack/src/bacnet/datalink/datalink.c",
        //"../bacnet-stack/src/bacnet/datalink/datalink.h",
        "../bacnet-stack/src/bacnet/datalink/dlenv.c",
        //"../bacnet-stack/src/bacnet/datalink/dlenv.h",
        //"../bacnet-stack/src/bacnet/datalink/dlmstp.h",
        //"../bacnet-stack/src/bacnet/datalink/ethernet.h",
        // "../bacnet-stack/src/bacnet/datalink/mstp.c",
        //"../bacnet-stack/src/bacnet/datalink/mstpdef.h",
        //"../bacnet-stack/src/bacnet/datalink/mstp.h",
        "../bacnet-stack/src/bacnet/datalink/mstptext.c",
        //"../bacnet-stack/src/bacnet/datalink/mstptext.h",
        "../bacnet-stack/src/bacnet/datetime.c",
        //"../bacnet-stack/src/bacnet/datetime.h",
        "../bacnet-stack/src/bacnet/dcc.c",
        //"../bacnet-stack/src/bacnet/dcc.h",
        "../bacnet-stack/src/bacnet/event.c",
        //"../bacnet-stack/src/bacnet/event.h",
        "../bacnet-stack/src/bacnet/get_alarm_sum.c",
        //"../bacnet-stack/src/bacnet/get_alarm_sum.h",
        "../bacnet-stack/src/bacnet/getevent.c",
        //"../bacnet-stack/src/bacnet/getevent.h",
        "../bacnet-stack/src/bacnet/hostnport.c",
        //"../bacnet-stack/src/bacnet/hostnport.h",
        "../bacnet-stack/src/bacnet/iam.c",
        //"../bacnet-stack/src/bacnet/iam.h",
        "../bacnet-stack/src/bacnet/ihave.c",
        //"../bacnet-stack/src/bacnet/ihave.h",
        "../bacnet-stack/src/bacnet/indtext.c",
        //"../bacnet-stack/src/bacnet/indtext.h",
        "../bacnet-stack/src/bacnet/lighting.c",
        //"../bacnet-stack/src/bacnet/lighting.h",
        "../bacnet-stack/src/bacnet/lso.c",
        //"../bacnet-stack/src/bacnet/lso.h",
        "../bacnet-stack/src/bacnet/memcopy.c",
        //"../bacnet-stack/src/bacnet/memcopy.h",
        "../bacnet-stack/src/bacnet/npdu.c",
        //"../bacnet-stack/src/bacnet/npdu.h",
        "../bacnet-stack/src/bacnet/property.c",
        //"../bacnet-stack/src/bacnet/property.h",
        "../bacnet-stack/src/bacnet/proplist.c",
        //"../bacnet-stack/src/bacnet/proplist.h",
        "../bacnet-stack/src/bacnet/ptransfer.c",
        //"../bacnet-stack/src/bacnet/ptransfer.h",
        "../bacnet-stack/src/bacnet/rd.c",
        //"../bacnet-stack/src/bacnet/rd.h",
        "../bacnet-stack/src/bacnet/readrange.c",
        //"../bacnet-stack/src/bacnet/readrange.h",
        "../bacnet-stack/src/bacnet/reject.c",
        //"../bacnet-stack/src/bacnet/reject.h",
        "../bacnet-stack/src/bacnet/rp.c",
        //"../bacnet-stack/src/bacnet/rp.h",
        "../bacnet-stack/src/bacnet/rpm.c",
        //"../bacnet-stack/src/bacnet/rpm.h",
        "../bacnet-stack/src/bacnet/timestamp.c",
        //"../bacnet-stack/src/bacnet/timestamp.h",
        "../bacnet-stack/src/bacnet/timesync.c",
        //"../bacnet-stack/src/bacnet/timesync.h",
        //"../bacnet-stack/src/bacnet/version.h",
        "../bacnet-stack/src/bacnet/whohas.c",
        //"../bacnet-stack/src/bacnet/whohas.h",
        "../bacnet-stack/src/bacnet/whois.c",
        //"../bacnet-stack/src/bacnet/whois.h",
        "../bacnet-stack/src/bacnet/wp.c",
        //"../bacnet-stack/src/bacnet/wp.h",
        "../bacnet-stack/src/bacnet/wpm.c",
        //"../bacnet-stack/src/bacnet/wpm.h",
    ];

    src_files.push("../bacnet-stack/src/bacnet/basic/object/gateway/gw_device.c");

    let ports_files = vec![
        // "../bacnet-stack/ports/linux/bacport.h",
        "../bacnet-stack/ports/linux/datetime-init.c",
        "../bacnet-stack/ports/linux/bip-init.c",
        "../bacnet-stack/ports/linux/mstimer-init.c",
    ];

    let mut builder = cc::Build::new();

    builder
        // .define("BACNET_STACK_BUILD_APPS", None)     // "build apps"
        .define("BAC_ROUTING", None)                 // "enable bac routing"
        .define("BACNET_PROPERTY_LISTS", None)       // "enable property lists"
        //.define("BACNET_BUILD_PIFACE_APP", "OFF")                     // "compile the piface app"
        .define("BACNET_BUILD_BACPOLL_APP", None)   // "compile the bacpoll app"
        //.define("BACDL_ETHERNET", "OFF")                              // "compile with ethernet support"
        //.define("BACDL_MSTP", "OFF")                                  // "compile with mstp support"
        //.define("BACDL_ARCNET", "OFF")                                // "compile with arcnet support"
        .define("BACDL_BIP", None)                            // "compile with ip support"
        //.define("BACDL_BIP6", "OFF")                                  // "compile with ipv6 support"
        //.define("BACDL_NONE", "OFF")                                  // "compile without datalink"
        .define("PRINT_ENABLED",None)
        .define("BACNET_STACK_STATIC_DEFINE", None)
        .define("BACNET_PROTOCOL_REVISION", "19");

    builder.include("../bacnet-stack/src");

    for file in src_files {
        builder.file(file);
    }

    builder.include("../bacnet-stack/src/bacnet/datalink")
        .include("../bacnet-stack/ports/linux");

    for file in ports_files {
        builder.file(file);
    }
    builder.compile("bacnet");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // let lib_path = out_path.join("libbacnet.a").into_os_string().into_string().unwrap();
    // let lib_path_str = lib_path.into_os_string().into_string().unwrap();

    // println!("cargo:rustc-link-search={}",out_path.into_os_string().into_string().unwrap());
    // println!("cargo:rustc-link-lib=bacnet");
}
