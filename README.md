Overview

This project is a Rust application that demonstrates how to securely handle sensitive data in memory. It generates a random password, stores it in the process heap, zeroes the memory after use, and finally frees the allocated memory. The project also includes basic encryption and decryption of strings using a custom key.
Features

    Generates a random 32-character alphanumeric password.
    Allocates memory for the password in the process heap.
    Copies the password to the allocated memory.
    Uses inline assembly to call RtlZeroMemory to zero the memory.
    Encrypts and decrypts strings using a custom key.
    Frees the allocated memory after use.
    Includes debugging pauses to inspect memory using tools like x64dbg.

Prerequisites

    Rust programming language: Install Rust
    A Windows operating system to use the WinAPI functions.
    x64dbg or any other debugger (optional, for debugging purposes).

Dependencies

    rand: For generating the random password.
    winapi: For accessing Windows API functions.
    RtlZeroPoc: For string encryption and decryption.

To add these dependencies, include the following in your Cargo.toml:

toml
```bash
[dependencies]
rand = "0.8.4"
winapi = { version = "0.3", features = ["heapapi", "libloaderapi", "ntdll"] }
RtlZeroPoc = { path = "../path/to/RtlZeroPoc" } # Adjust the path as needed
```

Usage

    Clone the repository.
    Navigate to the project directory.
    Run the application using cargo run.

Example


```bash
git clone https://github.com/your-username/your-repo.git
cd your-repo
cargo run
```
Code Explanation
Main Function

The main function is the entry point of the application. It generates a random password, allocates memory in the heap for the password, and handles the memory securely.

    Generate Password: A random 32-character alphanumeric password is generated.
    Allocate Memory: The process heap is obtained using GetProcessHeap, and memory is allocated using HeapAlloc.
    Copy Password: The password is copied to the allocated memory.
    Encrypt and Load Kernel32.dll: kernel32.dll is loaded, and the address of RtlZeroMemory is obtained.
    Zero Memory: Inline assembly is used to call RtlZeroMemory to zero the memory.
    Free Memory: The allocated memory is freed using HeapFree.

Debugging Pauses

The function press_enter_to_continue is used to pause the execution at various points for debugging purposes.
Encryption and Decryption

The obst! macro is used to decrypt strings. The ENCRYPTION_KEY is a constant key used for encryption and decryption.
Security Considerations

    The password is securely zeroed in memory after use to prevent sensitive data from lingering in memory.
    The use of inline assembly ensures direct control over the memory zeroing process.