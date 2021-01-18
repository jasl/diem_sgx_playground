extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::str;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
// const ENCLAVE_OUTPUT_BUF_MAX_LEN: usize = 32760 as usize;

extern {
    fn ecall_main(
        eid: sgx_enclave_id_t, retval: *mut sgx_status_t
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = option_env!("SGX_DEBUG").unwrap_or("1");

    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t {flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       if debug == "0" { 0 } else { 1 },
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;
    unsafe {
        ecall_main(enclave.geteid(), &mut retval);
    };
    match retval {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", retval.as_str());
            return;
        }
    }
    enclave.destroy();
}
