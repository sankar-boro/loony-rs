use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move {
        let mut file = File::open("./error.log").unwrap();
        let chunk_size = 500;

        loop {
            let mut chunk = Vec::with_capacity(chunk_size);
            let n = file
                .by_ref()
                .take(chunk_size as u64)
                .read_to_end(&mut chunk)
                .unwrap();
            println!("chunk: {:?}", String::from_utf8_lossy(&chunk));
            if n == 0 {
                break;
            }

            if n < chunk_size {
                break;
            }
        }
    });

    handle.await.unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
