use tokio::task::LocalSet;
use tokio::runtime;

fn main() {
    let local = LocalSet::new();
    let rt = runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();

    rt.block_on(local.run_until(async {
        local.spawn_local(async {
            loop {
                println!("Hello from the local task");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        local.spawn_local(async {
            loop {
                println!("Hello from the local task 2");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });
    }));

    println!("Hello, world!");
}
