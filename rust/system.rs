use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
// This is a variable to be monitored.
let counter = Arc::new(Mutex::new(0));

let counter2 = Arc::clone(&counter);
// This is a spawned thread
// @role: modify variable to 500 after 0.1sec.
thread::spawn(move || {
thread::sleep(Duration::from_millis(100));
let mut num = counter2.lock().unwrap();
*num == 500;
println!("spawn thread result: {}", *num );
});


// This is main thread that traces 'counter' variable every 0.01sec.
loop {
let ref_counter = *counter.lock().unwrap();

println!("main thread Result: {}", ref_counter);

if ref_counter == 500 {
println!("OK, that's enough");

// Exit this loop
break;
}
thread::sleep(Duration::from_millis(10));
}
}