use improv::*;

#[tokio::test]
async fn ask() {
    #[derive(Default)]
    struct A(usize);
    impl Actor for A {}

    #[async_trait]
    impl Receive<M> for A {
        async fn receive(&mut self, _: M, cx: &mut Context<Self>) -> usize {
            self.0 += 1;
            if self.0 == 3 {
                cx.stop();
            }
            self.0
        }
    }

    struct M;
    impl Message for M {
        type Response = usize;
    }

    let addr = improv::spawn(|_| A::default());

    assert_eq!(addr.ask(M).await, Ok(1));
    assert_eq!(addr.ask(M).await, Ok(2));
    assert_eq!(addr.ask(M).await, Ok(3));
    assert!(addr.ask(M).await.is_err());
}

#[cfg(feature = "test-util")]
#[tokio::test]
async fn probe_ok() {
    struct A;
    impl Actor for A {}

    #[async_trait]
    impl<T: Actor> Receive<M<T>> for A
    where
        T: Receive<Noop>,
    {
        async fn receive(&mut self, M(addr): M<T>, _: &mut Context<Self>) {
            let _ = addr.tell(Noop);
        }
    }

    struct B;
    impl Actor for B {}

    #[async_trait]
    impl Receive<Noop> for B {
        async fn receive(&mut self, _: Noop, _: &mut Context<Self>) {}
    }

    struct M<T: Actor>(Addr<T>);
    impl<T: Actor> Message for M<T> {
        type Response = ();
    }

    struct Noop;
    impl Message for Noop {
        type Response = ();
    }

    let addr = improv::spawn(|_| A);

    let mut probe = Probe::<B>::new();
    assert!(addr.tell(M(probe.addr())).is_ok());
    probe.receive::<Noop>().await;
}

#[cfg(feature = "test-util")]
#[tokio::test]
#[should_panic]
async fn probe_err() {
    struct A;
    impl Actor for A {}

    #[async_trait]
    impl<T: Actor> Receive<M<T>> for A
    where
        T: Receive<Noop>,
    {
        async fn receive(&mut self, M(addr): M<T>, _: &mut Context<Self>) {
            let _ = addr.tell(Noop);
        }
    }

    struct B;
    impl Actor for B {}

    #[async_trait]
    impl Receive<Noop> for B {
        async fn receive(&mut self, _: Noop, _: &mut Context<Self>) {}
    }

    #[async_trait]
    impl Receive<UhOh> for B {
        async fn receive(&mut self, _: UhOh, _: &mut Context<Self>) {}
    }

    struct M<T: Actor>(Addr<T>);
    impl<T: Actor> Message for M<T> {
        type Response = ();
    }

    struct Noop;
    impl Message for Noop {
        type Response = ();
    }

    struct UhOh;
    impl Message for UhOh {
        type Response = ();
    }

    let addr = improv::spawn(|_| A);

    let mut probe = Probe::<B>::new();
    assert!(addr.tell(M(probe.addr())).is_ok());
    probe.receive::<UhOh>().await;
}
