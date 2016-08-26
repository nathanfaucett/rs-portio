

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, &str);
}

impl<F> Handler for F where F: Send + Sync + 'static + Fn(&str) {
    fn handle(&self, string: &str) {
        (*self)(string)
    }
}
