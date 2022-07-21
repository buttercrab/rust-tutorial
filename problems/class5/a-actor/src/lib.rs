//! Simple async actor model

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Actor: Send + Sized + 'static {
    async fn started(&mut self, _ctx: &mut Context<Self>) {}

    fn start(self) -> Addr<Self> {
        todo!()
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

pub struct Context<A: Actor> {
    inner: Arc<Inner<A>>,
}

impl<A: Actor> Context<A> {
    #[allow(dead_code)]
    fn new(tx: Sender<Box<dyn Envelope<A> + Send>>, stop_tx: Sender<()>) -> Context<A> {
        Context {
            inner: Arc::new(Inner { tx, stop_tx }),
        }
    }

    pub fn addr(&self) -> Addr<A> {
        Addr {
            inner: self.inner.clone(),
        }
    }

    pub async fn stop(&self) {
        let _ = self.inner.stop_tx.send(()).await;
    }
}

#[derive(Clone)]
pub struct Addr<A: Actor> {
    inner: Arc<Inner<A>>,
}

impl<A: Actor> Addr<A> {
    pub async fn send<M>(&self, msg: M)
    where
        A: Handler<M>,
        M: Send + 'static,
    {
        self.inner.send(msg).await;
    }
}

#[async_trait]
pub trait Handler<M>
where
    Self: Actor,
{
    async fn handle(&mut self, msg: M, ctx: &mut Context<Self>);
}

#[async_trait]
trait Envelope<A: Actor> {
    async fn handle(&mut self, act: &mut A, ctx: &mut Context<A>);
}

struct Message<M> {
    msg: Option<M>,
}

#[async_trait]
impl<A, M> Envelope<A> for Message<M>
where
    A: Actor + Handler<M>,
    M: Send,
{
    async fn handle(&mut self, act: &mut A, ctx: &mut Context<A>) {
        act.handle(self.msg.take().unwrap(), ctx).await;
    }
}

struct Inner<A: Actor> {
    tx: Sender<Box<dyn Envelope<A> + Send>>,
    stop_tx: Sender<()>,
}

impl<A: Actor> Inner<A> {
    async fn send<M>(&self, msg: M)
    where
        A: Handler<M>,
        M: Send + 'static,
    {
        let _ = self.tx.send(Box::new(Message { msg: Some(msg) })).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Foo {
        data: usize,
    }

    impl Actor for Foo {}

    impl Foo {
        fn new(data: usize) -> Foo {
            Foo { data }
        }
    }

    struct Msg {
        data: usize,
    }

    impl Msg {
        fn new(data: usize) -> Msg {
            Msg { data }
        }
    }

    #[async_trait]
    impl Handler<Msg> for Foo {
        async fn handle(&mut self, msg: Msg, _ctx: &mut Context<Foo>) {
            self.data += msg.data;
        }
    }

    struct Check {
        data: usize,
    }

    impl Check {
        fn new(data: usize) -> Check {
            Check { data }
        }
    }

    #[async_trait]
    impl Handler<Check> for Foo {
        async fn handle(&mut self, msg: Check, _ctx: &mut Context<Foo>) {
            assert_eq!(self.data, msg.data);
        }
    }

    #[tokio::test]
    async fn simple_test() {
        let foo = Foo::new(0).start();
        foo.send(Msg::new(10)).await;
        foo.send(Check::new(10)).await;
    }
}
