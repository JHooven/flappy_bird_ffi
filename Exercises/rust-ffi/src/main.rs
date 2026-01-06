use std::ffi::c_int;  
use std::ffi::c_void;
use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

unsafe extern "C" {
	fn add_numbers(a: c_int, b: c_int) -> c_int;

	fn handle_data(ptr: *const c_void);

    // Function that returns a void* (e.g., memory allocation)
    fn create_buffer() -> *mut c_void;

    // Function that takes nothing and returns void
    fn reset_system() -> ();

	fn free(ptr: *const c_void); // Calls C's free function

	fn get_ascii_string() -> *const c_char;

	fn print_message(msg: *const c_char); 
}

fn main() {
	let sum = unsafe { add_numbers(3, 4) };
	println!("Sum is: {}", sum);

	let mut num = 100;
    let num_ptr = &mut num as *mut i32 as *mut c_void; // Convert to void*
    unsafe {
        handle_data(num_ptr); // Call C function
    }

    // Example 2: Getting a void* from C
    let buffer_ptr: *mut c_void;
    unsafe {
        buffer_ptr = create_buffer();
        if !buffer_ptr.is_null() {
            let int_ptr = buffer_ptr as *mut i32; // Convert back
            println!("Rust received buffer with value: {}", *int_ptr);
            
            // Free memory using C's free function
            free(buffer_ptr);
        } else {
            println!("Failed to create buffer.");
        }
    }

    // Example 3: Calling a function that takes no arguments and returns void
    unsafe {
        reset_system();
	}

	unsafe {
        
		let c_str = CStr::from_ptr(get_ascii_string());
		match c_str.to_str() {
             Ok(valid_str) => println!("Valid UTF-8: {}", valid_str),
             Err(_) => println!("Invalid UTF-8 received!"),
         }
	}

	let rust_string = String::from("Hello Rust!");

    // Convert Rust String to a C-compatible CString
    let c_string = CString::new(rust_string).expect("CString conversion failed");
    
    unsafe {
        print_message(c_string.as_ptr());  // Pass `char *` to C
		print_message(CString::new("").unwrap().as_ptr());
    }
    
}
