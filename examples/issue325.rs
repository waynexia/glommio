use glommio::{Local, LocalExecutor};
use std::sync::mpsc::channel;
use tokio_1_4::sync::oneshot::{channel as oneshot, Sender as Callback};

fn main() {
    let (tx, rx) = channel::<Callback<()>>();
    let handle = std::thread::spawn(|| {
        let ex = LocalExecutor::default();
        ex.run(async move {
            while let Ok(o_tx) = rx.recv() {
                println!("received");
                Local::local(async {
                    println!("polled");
                    o_tx.send(()).unwrap();
                })
                .detach();
            }
        })
    });

    let (o_tx, o_rx) = oneshot();
    tx.send(o_tx).unwrap();

    let rt = tokio_1_4::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(o_rx).unwrap();

    drop(tx);
    handle.join().unwrap();
}
