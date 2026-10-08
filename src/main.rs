
#[cfg(target_arch = "x86_64")]
fn asm_int() ->u64 {
    let lo: u32 ;   // младшие 32 бита TSC
    let hi: u32 ;   // старшие 32 бита TSC

    unsafe {
        // Макрос asm! из модуля core::arch позволяет вставлять ассемблерный 
        // код прямо в Rust.        
        core::arch::asm!(
            "rdtsc",    // Read Time-Stamp Counter (регистр процессора, который увеличивается на 1 каждый такт)
            out("eax") lo,  // Extend AX регистр, младшие 32 бита TSC
            out("edx" ) hi, // Extend DX регистр, старшие 32 бита TSC
        ) ;
    }

    (hi as u64) << 32 | lo as u64   // итоговое значение счётчика тактов
}

fn main() {
    println!("Time-Stamp Counter: {}", asm_int());  // Out: Time-Stamp Counter: 9235531988396
}
