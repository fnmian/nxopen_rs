use nxopen_rs::nx_println;


#[unsafe(no_mangle)]
pub extern "C" fn ufusr(){
    nx_println!("{}",1244);
    
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}