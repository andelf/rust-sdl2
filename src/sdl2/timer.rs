use std::libc::{uint32_t, uint64_t, c_void, c_int};
use std::cast;
use std::ptr;

pub mod ll {

    use std::libc::{uint32_t, uint64_t, c_void, c_int};

    pub type SDL_TimerCallback =
        ::std::option::Option<extern "C" fn(arg1: uint32_t, arg2: *c_void)
                                            -> uint32_t>;

    pub type SDL_TimerID = c_int;
    //SDL_timer.h
    extern "C" {
        pub fn SDL_GetTicks() -> uint32_t;
        pub fn SDL_GetPerformanceCounter() -> uint64_t;
        pub fn SDL_GetPerformanceFrequency() -> uint64_t;
        pub fn SDL_Delay(ms: uint32_t);

        pub fn SDL_AddTimer(interval: uint32_t, callback: SDL_TimerCallback,
                            param: *c_void) -> SDL_TimerID;
        pub fn SDL_RemoveTimer(id: SDL_TimerID) -> c_int;
    }
    //TODO: Figure out what to do with the timer callback functions
}

pub fn get_ticks() -> uint {
    unsafe { ll::SDL_GetTicks() as uint }
}

pub fn get_performance_counter() -> u64 {
    unsafe { ll::SDL_GetPerformanceCounter() }
}

pub fn get_performance_frequency() -> u64 {
    unsafe { ll::SDL_GetPerformanceFrequency() }
}

pub fn delay(ms: uint) {
    unsafe { ll::SDL_Delay(ms as u32) }
}

struct CallbackParam<'a> {
    cb: 'a |uint, *c_void| -> uint,
    payload: *c_void
}

// pub struct Timer<'a> {
//     raw: ll::SDL_TimerID,
//     cb: |uint, *c_void|:'a -> uint,
//     payload: *c_void
// }

// impl<'a> Timer<'a> {
//     pub fn new<'a>(delay: u32, callback: 'a |uint, *c_void| -> uint, param: *c_void) -> Timer {
//         let param = CallbackParam { cb: callback, payload: param };

//         unsafe {
//             let timer_id = ll::SDL_AddTimer(delay as u32, Some(callback_function), cast::transmute(&param));
//             Timer { raw: timer_id, param: ~param }
//         }
//     }
// }



extern "C" fn callback_function(interval: u32, param: *c_void) -> u32 {
    println!("!!cb!!");
    let param : &CallbackParam = unsafe { cast::transmute(param) };
    let ref cb = param.cb;
    let ref payload = param.payload;

    //println!("addr => {:?}", &param);
    println!("callback() got cb: {:?}", cb);
    (*cb)(0, ptr::null());
    (*cb)(interval as uint, *payload) as u32
}

pub fn add_timer(delay: uint, callback: |uint, *c_void| -> uint, param: *c_void) -> int {
    let ~param = ~CallbackParam { cb: callback, payload: param };
    println!("add_timer() got cb: {:?}", param.cb);
    (param.cb)(0, ptr::null());
    println!("addr => {}", &param as *CallbackParam);
    unsafe {
        ll::SDL_AddTimer(delay as u32, Some(callback_function), cast::transmute(&param)) as int
    }
}
