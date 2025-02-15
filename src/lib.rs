use neon::prelude::*;

use objc2::extern_class;
use objc2_foundation::{NSObject, NSString};
use objc2::rc::autoreleasepool;
use objc2::msg_send;
use objc2::ClassType;
use io_kit_sys::{IOServiceMatching, IOServiceGetMatchingService, IORegistryEntryCreateCFProperty, IOObjectRelease};
use core_foundation::base::{CFRelease, TCFType};
use core_foundation::string::{CFString};
use std::os::raw::{c_char};

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq)]
    pub struct NSHost;
);

impl NSHost {
    pub fn current_host() -> Option<&'static Self> {
        unsafe {
            let host: *mut Self = msg_send![Self::class(), currentHost];
            if host.is_null() {
                None
            } else {
                Some(&*host)
            }
        }
    }

    pub fn localized_name(&self) -> Option<&NSString> {
        unsafe {
            let name: *mut NSString = msg_send![self, localizedName];
            if name.is_null() {
                None
            } else {
                Some(&*name)
            }
        }
    }
}

fn get_computer_name() -> Option<String> {
    autoreleasepool(|_| {
        let host = NSHost::current_host()?;
        let ns_string = host.localized_name()?;
        // Convert to a Rust String.
        Some(ns_string.to_string())
    })
}

fn get_computer_uuid() -> Option<String> {
    unsafe {
        let service_name = b"IOPlatformExpertDevice\0";
        let matching = IOServiceMatching(service_name.as_ptr() as *const c_char);
        if matching.is_null() {
            return None;
        }

        let platform_expert = IOServiceGetMatchingService(0, matching);
        if platform_expert == 0 {
            return None;
        }

        let key: CFString = CFString::from_static_string("IOPlatformUUID");
        let cf_uuid = IORegistryEntryCreateCFProperty(platform_expert, key.as_concrete_TypeRef(), std::ptr::null_mut(), 0);
        IOObjectRelease(platform_expert);

        if cf_uuid.is_null() {
            return None;
        }

        let uuid_cf: CFString = CFString::wrap_under_get_rule(cf_uuid as *mut _);
        let uuid = uuid_cf.to_string();

        CFRelease(cf_uuid as *mut _);
        Some(uuid)
    }
}

fn get_hardware_uuid_js(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(get_computer_uuid().unwrap()))
}

fn get_computer_name_js(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(get_computer_name().unwrap()))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getHardwareUuid", get_hardware_uuid_js)?;
    cx.export_function("getComputerName", get_computer_name_js)?;
    Ok(())
}
