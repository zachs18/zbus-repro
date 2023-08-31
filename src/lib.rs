#![allow(unused_must_use)]

// async fn make_connection() -> zbus::Result<zbus::Connection> {
//     todo!()
// }

macro_rules! make_many_funcs {
    ($( $name:ident $wrapper_name:ident ;)*) => {
        $(
            async fn $name() {
                let _ = zbus::Connection::session().await;
                // let _ = tokio::time::sleep(std::time::Duration::default()).await;
                // let _ = make_connection().await;
                // let _ = std::future::pending::<zbus::Result<zbus::Connection>>().await;
            }
            pub fn $wrapper_name() {
                let future = $name();
                Box::pin(future);
            }
        )*
    };
}

#[cfg(feature = "1")]
make_many_funcs! {
    a aa;
}
#[cfg(feature = "4")]
make_many_funcs! {
    b bb;
    c cc;
    d dd;
}
#[cfg(feature = "8")]
make_many_funcs! {
    e ee;
    f ff;
    g gg;
    h hh;
}
#[cfg(feature = "12")]
make_many_funcs! {
    i ii;
    j jj;
    k kk;
    l ll;
}
#[cfg(feature = "16")]
make_many_funcs! {
    m mm;
    n nn;
    o oo;
    p pp;
}
