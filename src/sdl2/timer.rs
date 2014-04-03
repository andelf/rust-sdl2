use std::libc::{c_void, uint32_t};
use std::cast;
use std::rc::Rc;
use std::mem;

#[allow(non_camel_case_types)]
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

struct CallbackParam<'a, T> {
    cb: 'a |uint, T| -> uint,
    param: T,
}

pub struct Timer<'a, T> {
    delay: uint,
    raw: ll::SDL_TimerID,
    closure: Rc<CallbackParam<'a, T>>
}

impl<'a, T> Timer<'a, T> {
    pub fn new<'a>(delay: uint, callback: 'a |uint, T| -> uint, cbparam: T) -> Timer<'a, T> {
        let cb_func = match mem::size_of::<T>() {
            1  => callback_function_1,
            2  => callback_function_2,
            4  => callback_function_4,
            6  => callback_function_6,
            8  => callback_function_8,
            16 => callback_function_16,
            32 => callback_function_32,
            _ => unimplemented!()
        };
        // use Rc box to store closure and param
        let c_param = Rc::new(CallbackParam { cb: callback, param: cbparam });
        let timer_id = unsafe { ll::SDL_AddTimer(delay as u32, Some(cb_func), cast::transmute(c_param.deref())) };
        Timer { delay: delay, raw: timer_id, closure: c_param }
    }

    pub fn remove(&self) {
        unsafe { ll::SDL_RemoveTimer(self.raw) };
    }

}

macro_rules! gen_callback_func_with_param_size(
    ($name:ident, $size:expr) => (
        extern "C" fn $name(interval: uint32_t, param: *c_void) -> uint32_t {
            let param_wrapper : &CallbackParam<[u8, ..$size]> = unsafe { cast::transmute(param) };
            let ref func = param_wrapper.cb;
            let ref fparam = param_wrapper.param;
            (*func)(interval as uint, *fparam) as uint32_t
        }
    )
)

gen_callback_func_with_param_size!(callback_function_1, 1)
gen_callback_func_with_param_size!(callback_function_2, 2)
gen_callback_func_with_param_size!(callback_function_4, 4)
gen_callback_func_with_param_size!(callback_function_6, 6)
gen_callback_func_with_param_size!(callback_function_8, 8)
gen_callback_func_with_param_size!(callback_function_16, 16)
gen_callback_func_with_param_size!(callback_function_32, 32)
