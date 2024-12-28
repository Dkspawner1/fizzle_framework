use crate::threading::job::JobPtr;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::thread::{spawn, JoinHandle};

pub struct JobSystem {
    job_queue: Arc<Mutex<VecDeque<JobPtr>>>,
    condition: Arc<Condvar>,
    threads: Vec<thread::JoinHandle<()>>,
    should_exit: Arc<Mutex<bool>>,
    active_jobs: Arc<Mutex<usize>>,
}

impl JobSystem {
    pub fn new(thread_count: usize) -> Self {
        let job_queue = Arc::new(Mutex::new(VecDeque::new()));
        let condition = Arc::new(Condvar::new());
        let should_exit = Arc::new(Mutex::new(false));
        let active_jobs = Arc::new(Mutex::new(0));

        let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let job_queue: Arc<Mutex<VecDeque<JobPtr>>> = Arc::clone(&job_queue);
            let condition: Arc<Condvar> = Arc::clone(&condition);
            let should_exit: Arc<Mutex<bool>> = Arc::clone(&should_exit);
            let active_jobs: Arc<Mutex<usize>> = Arc::clone(&active_jobs);

            Vec::push(
                &mut threads,
                spawn(move || Self::worker_thread(job_queue, condition, should_exit, active_jobs)),
            );
        }
        JobSystem {
            job_queue,
            condition,
            threads,
            should_exit,
            active_jobs,
        }
    }

    pub fn add_job(&self, job: JobPtr) {
        {
            let mut queue: MutexGuard<VecDeque<JobPtr>> = self.job_queue.lock().unwrap();
            VecDeque::push_back(&mut queue, job);
            *Mutex::lock(&self.active_jobs).unwrap() += 1;
        }
        Condvar::notify_one(&self.condition);
    }

    pub fn wait_for_all_jobs(&self) {
        while *Mutex::lock(&self.active_jobs).unwrap() > 0 {
            thread::yield_now();
        }
    }

    fn worker_thread(
        job_queue: Arc<Mutex<VecDeque<JobPtr>>>,
        condition: Arc<Condvar>,
        should_exit: Arc<Mutex<bool>>,
        active_jobs: Arc<Mutex<usize>>,
    ) {
        loop {
            let job: Option<JobPtr> = {
                let mut queue: MutexGuard<VecDeque<JobPtr>> = Mutex::lock(&job_queue).unwrap();
                queue = condition
                    .wait_while(queue, |queue: &mut VecDeque<JobPtr>| {
                        VecDeque::is_empty(queue) && !*should_exit.lock().unwrap()
                    })
                    .unwrap();

                if *should_exit.lock().unwrap() && VecDeque::is_empty(&queue) {
                    break;
                }

                queue.pop_front()
            };

            if let Some(job) = job {
                job.execute();
                *active_jobs.lock().unwrap() -= 1;
            }
        }
    }
}

impl Drop for JobSystem {
    fn drop(&mut self) {
        *self.should_exit.lock().unwrap() = true;
        self.condition.notify_all();
        for thread in self.threads.drain(..) {
            thread.join().unwrap();
        }
    }
}
