extern crate rand;
extern crate winapi;
use std::arch::asm;
use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;
use rand::{distributions::Alphanumeric, Rng};
use winapi::ctypes::c_void;
use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use RtlZeroPoc::{decrypt_string, ENCRYPTION_KEY, obst};
fn main() {
    // Generar una contraseña aleatoria
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    println!("Generated password: {}", password);

    // Convertir la contraseña a bytes y asignarla en el heap
    let password_bytes = password.as_bytes();
    let password_len = password_bytes.len();

    unsafe {
        // Obtener el heap del proceso
        let heap = GetProcessHeap();
        if heap.is_null() {
            panic!("Failed to get process heap");
        }

        // Asignar memoria en el heap para la contraseña
        let password_ptr = HeapAlloc(heap, 0, password_len) as *mut u8;
        if password_ptr.is_null() {
            panic!("Failed to allocate memory for password");
        }
        println!("Memory allocated for password at address: {:?}", password_ptr);

        // Copiar la contraseña a la memoria asignada
        ptr::copy_nonoverlapping(password_bytes.as_ptr(), password_ptr, password_len);

        // Imprimir la contraseña desde la memoria asignada (para depuración)
        let password_in_memory = std::slice::from_raw_parts(password_ptr, password_len);
        println!(
            "Password in memory before zeroing: {:?}",
            std::str::from_utf8(password_in_memory).unwrap()
        );

        // Pausa para depuración: verifica la dirección de la contraseña en x64dbg
        press_enter_to_continue();
        let encrypted_kernel: String = match obst!("kernel32.dll", ENCRYPTION_KEY) {
            Ok(s) => s,
            Err(e) => panic!("Decryption failed: {:?}", e),
        };
        // Cargar kernel32.dll para obtener la dirección de RtlZeroMemory
        let kernel32 = LoadLibraryA(CString::new(encrypted_kernel).unwrap().as_ptr());
        if kernel32.is_null() {
            panic!("Failed to load kernel32.dll");
        }
        let encrypted_zero: String = match obst!("RtlZeroMemory", ENCRYPTION_KEY) {
            Ok(s) => s,
            Err(e) => panic!("Decryption failed: {:?}", e),
        };

        // Obtener la dirección de RtlZeroMemory
        let rtl_zero_memory = GetProcAddress(kernel32, CString::new(encrypted_zero).unwrap().as_ptr());
        if rtl_zero_memory.is_null() {
            panic!("Failed to get address of RtlZeroMemory");
        }

        // Pausa para depuración: antes de llamar a RtlZeroMemory
        press_enter_to_continue();

        // Usar ensamblador inline para llamar a RtlZeroMemory
        asm!(
        "call {}",
        in(reg) rtl_zero_memory,
        in("rcx") password_ptr as *mut c_void, // primer argumento (destino)
        in("rdx") password_len, // segundo argumento (longitud)
        );

        println!("Memory zeroed.");

        // Intentar imprimir la contraseña nuevamente (debería estar borrada)
        let password_in_memory_after_zeroing = std::slice::from_raw_parts(password_ptr, password_len);
        println!(
            "Password in memory after zeroing: {:?}",
            std::str::from_utf8(password_in_memory_after_zeroing).unwrap_or("Memory has been zeroed")
        );

        // Pausa para depuración: después de llamar a RtlZeroMemory
        press_enter_to_continue();

        // Liberar la memoria asignada
        HeapFree(heap, 0, password_ptr as *mut c_void);
        println!("Memory freed.");
    }

    println!("Program finished.");
    press_enter_to_continue();
}

fn press_enter_to_continue() {
    let mut stdout = io::stdout();
    write!(stdout, "Press ENTER to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}
