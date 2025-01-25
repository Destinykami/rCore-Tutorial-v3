use crate::batch::{USER_STACK, USER_STACK_SIZE, APP_BASE_ADDRESS, APP_SIZE_LIMIT};
const FD_STDOUT: usize = 1;
fn check_addr_legality(slice: &[u8]) -> Option<isize> {
    let app_start = slice.as_ptr() as usize;
    let app_size = slice.len();
    //检查地址是否越界
    if app_start<=APP_BASE_ADDRESS || app_start+app_size >=APP_BASE_ADDRESS+APP_SIZE_LIMIT|| app_start + app_size >= APP_BASE_ADDRESS + APP_SIZE_LIMIT{
        None
    }
    // if !((app_start >= APP_BASE_ADDRESS &&
    //     app_start + app_size <= APP_BASE_ADDRESS + APP_SIZE_LIMIT) ||
    //     (app_start + app_size <=  USER_STACK.get_sp()  &&
    //     app_start >=  USER_STACK.get_sp() - USER_STACK_SIZE )) {
    //     None
    // } 
    else {
        Some(app_size as isize)
    }
}
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            match check_addr_legality(slice) {
                None => -1 as isize,
                Some(w_len) => {
                    let str = core::str::from_utf8(slice).unwrap();
                    print!("{}", str);
                    w_len
                }
            }
        },
        _ => {
            println!("Unsupported fd in sys_write!");
            -1 as isize
        }
    }
}