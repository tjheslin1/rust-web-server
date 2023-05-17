use std::thread;

#[derive(Debug, PartialEq)]
pub struct ThreadPool {
	workers: Vec<Worker>,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

#[derive(Debug, PartialEq)]
pub struct PoolCreationError {
	message: &'static str
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
			workers.push(Worker::new(id));
   	    }

        ThreadPool { workers }
    }

	pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
		match size {
			0 => Err(PoolCreationError {
				message: "Cannot create a pool of size 0!"
			}),
			_ => Ok(ThreadPool::new(size)),
		}
	}

	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
	{
		
	}
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn pool_creation_error() {
		let result = ThreadPool::build(0);

		assert!(result.is_err());

		let actual = result.unwrap_err();
		let expected = PoolCreationError { message: "Cannot create a pool of size 0!" };

		assert_eq!(actual, expected);
	}

	#[test]
	fn pool_creation() {
		let result = ThreadPool::build(2);

		assert!(result.is_ok());

		let actual = result.unwrap();
		let expected = ThreadPool { workers: vec![
			Worker { id: 0, thread: thread::spawn(|| {}) },
			Worker { id: 1, thread: thread::spawn(|| {}) },
		]};

		assert_eq!(actual, expected);
	}
}
