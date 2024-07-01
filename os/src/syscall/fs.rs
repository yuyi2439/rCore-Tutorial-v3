//! File and filesystem-related syscalls

use log::{debug, error};

use crate::batch::{get_current_app_addr, get_user_stack_sp, APP_BASE_ADDRESS, USER_STACK_SIZE};

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, addr: usize, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // check addr
            let curr_app_addr = get_current_app_addr();
            let app_addr_range =
                APP_BASE_ADDRESS..(APP_BASE_ADDRESS + curr_app_addr.1 - curr_app_addr.0);
            let user_stack_range = get_user_stack_sp() - USER_STACK_SIZE..get_user_stack_sp();

            debug!(
                "addr: {:#x}, [{:#x}, {:#x})",
                addr,
                APP_BASE_ADDRESS,
                (APP_BASE_ADDRESS + curr_app_addr.1 - curr_app_addr.0)
            );
            debug!("{:#x}", get_user_stack_sp());

            if !app_addr_range.contains(&addr) && !user_stack_range.contains(&addr) {
                error!("Want to use addr that don't belong to this app");
                return -1;
            }

            let buf = addr as *const u8;
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
