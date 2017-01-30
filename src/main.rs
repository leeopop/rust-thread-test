use std::sync;
use std::thread;

fn main() {
	let mut children = vec![];
	let counter = sync::Arc::new(sync::RwLock::new(0usize));
	
	for thread_id in 0..4 {
		let val = counter.clone();
		children.push(thread::spawn(move || {
					println!("Hello thread {}", thread_id);
					{
						println!("Current value (tid:{}), {}", thread_id, *val.read().unwrap());
					}
					{
						let mut write = val.write().unwrap();
						*write += thread_id;
						println!("Updated value (tid:{}) {}", thread_id, *write);
					}
		}));
	}
	
	for thread in children {
		let result = thread.join();
		if let Err(err) = result {
			println!("Error occurred! {:?}", err);
		}
	}
}