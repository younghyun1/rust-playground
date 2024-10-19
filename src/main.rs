// async
// fn main() {
//     let start = tokio::time::Instant::now();

//     let body = async {

//     };

//     {
//         return tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .expect("Failed to build Tokio Runtime.")
//             .block_on(body);
//     }
// }

// sync
fn main() {
}

// no_std
// #![no_std]
// #![no_main]

// use core::panic::PanicInfo;

// // Define the entry point
// #[no_mangle]
// pub extern "C" fn start() -> ! {
//     loop {}
// }

// // Define what happens on panic
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
