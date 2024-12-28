use std::sync::Arc;

pub struct Job
{
    function: Arc<dyn Fn() + Send + Sync + 'static>,
    description: String,
}

impl Job
{
    pub fn new <F>(function: F, description: String) -> Self
    where F: Fn() + Send + Sync + 'static,
    {
        Job
        {
            function: Arc::new(function),
            description,
        }
    }
    pub fn execute(&self)
    {
        (self.function)();
    }
}
pub type JobPtr = Arc<Job>;