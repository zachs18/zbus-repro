mod systemd;
use std::future::Future;
use crate::systemd::*;
use anyhow::Result;
use futures::{FutureExt, future::BoxFuture};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Hello, world!");
    Ok(())
}

// If I comment out these 3 functions, build times drop from 7s to 0.7s:

fn start_as_action(service_name: String) {
    let future = start(service_name);
    let boxed_future = Box::pin(future);
    run_action_boxed(boxed_future);
}
fn stop_as_action(service_name: String) {
    let future = stop(service_name);
    let boxed_future = Box::pin(future);
    run_action_boxed(boxed_future);
}
fn restart_as_action(service_name: String) {
    let future = restart(service_name);
    let boxed_future = Box::pin(future);
    run_action_boxed(boxed_future);
}

async fn start(service_name: String) -> Result<()> {
    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    manager_proxy
        .start_unit(service_name.into(), "replace".into())
        .await?;
    Ok(())
}

async fn stop(service_name: String) -> Result<()> {
    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    manager_proxy
        .stop_unit(service_name.into(), "replace".into())
        .await?;
    Ok(())
}

async fn restart(service_name: String) -> Result<()> {
    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    manager_proxy
        .restart_unit(service_name.into(), "replace".into())
        .await?;
    Ok(())
}

fn run_action_boxed(action: BoxFuture<'static, Result<()>>) {
    tokio::spawn(async move {
        let _ = action.await;
    });
}

fn run_action<Fut>(action: Fut)
where
    Fut: Future + Send + 'static,
{
    let action = action.boxed();
    tokio::spawn(async move {
        let _ = action.await;
    });
}
