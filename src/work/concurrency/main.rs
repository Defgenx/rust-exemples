use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{self, TryRecvError};

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let (sender, receiver) = mpsc::channel();
//    let mut handles = vec![];
//    let data_iter = data.clone().into_iter();
    for (i,_) in (0..3).enumerate() {
        let (data, sender) = (data.clone(), sender.clone());
        let _handle = thread::spawn(move || {
            println!("Sender add 1 to value and Send");
            let data = data.lock().unwrap();
            let handle_error = sender.send((data[i], data[i] + 1));
            if handle_error.is_err() {
                panic!(thread::current().id())
            }
            // Close Channel sender for this thread (for fun)
            println!("Close sender channel");
//            drop(sender);
            thread::current().id()
        });
        // Add handle thread to list
//        handles.push(handle);
    }
//    for handle in handles {
//        let result = handle.join();
//        println!("Thread encounter error ? {:?} for {:?}", result.is_err(), result.unwrap());
//    }

    for elt in receiver {
//    loop {
//        match receiver.try_recv() {
//            Ok(val) => {
//                let (prev_val, new_val) = val;
//                println!("Old val {} => New val {}", prev_val, new_val);
//            }
//            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => {
//                println!("Terminating.");
//                // Close receiver (for fun)
////                println!("Close receiver channel");
////                drop(receiver);
//                println!("All senders closed, closing channel.");
//                break;
//            }
        let (prev_val, new_val) = elt;
        println!("Old val {} => New val {}", prev_val, new_val);
        }
//    }
}
