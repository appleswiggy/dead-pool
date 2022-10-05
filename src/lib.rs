//! # Dead-Pool
//!
//! `dead-pool` provides a pool of threads waiting for a job.
//! Jobs are sent via `execute` method defined on `ThreadPool`.
pub use threadpool::ThreadPool;

mod threadpool;
mod worker;

#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn test1() {
        let pool = ThreadPool::new(4);
        for _ in 0..100 {
            pool.execute(|| {
                println!("Working");
            });
        }
    }
}
