use common::thread::{taskjoin::TaskJoin, JsChannel};
use rquickjs::{class::Trace, prelude::Rest, Function, Value};

#[rquickjs::class]
pub struct ThreadPool {
    pool: threadpool::ThreadPool,
    pool_count: usize,
}
impl<'js> Trace<'js> for ThreadPool {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::methods]
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let pool = threadpool::Builder::new().num_threads(size).build();
        return Self { pool, pool_count: size,};
    }
    pub fn pool_count(&self) -> usize {self.pool_count}
    pub fn spawn<'js>(&mut self, fun: Function<'js>, params: Rest<Value<'js>>) -> TaskJoin {
        let (fun_name, params_json) = super::setup_task(fun, params);
        let (sender, receiver) = futures_channel::oneshot::channel::<Option<String>>();
        self.pool.execute(|| {
            super::super::RUNTIME.with(|rt2| {
                let result = super::in_thread(rt2, params_json, fun_name, None);
                sender.send(result).unwrap();
            });
        });
        return TaskJoin::new(Some(receiver), None);
    }
    pub fn spawn_with_channel<'js>(&mut self, fun: Function<'js>, params: Rest<Value<'js>>) -> TaskJoin {
        let (fun_name, params_json) = super::setup_task(fun, params);
        let [channel0, channel1] = JsChannel::pair();
        let (sender, receiver) = futures_channel::oneshot::channel::<Option<String>>();
        self.pool.execute(|| {
            super::super::RUNTIME.with(|rt2| {
                let result = super::in_thread(rt2, params_json, fun_name, Some(channel0));
                sender.send(result).unwrap();
            });
        });
        return TaskJoin::new(Some(receiver), Some(channel1));
    }
}

