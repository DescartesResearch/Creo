use tokio::io::{AsyncReadExt, AsyncWriteExt};

const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub async fn copy(){
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut handles = tokio::task::JoinSet::new();
    handles.spawn(async move {
        let path = std::path::PathBuf::from(PROJECT_ROOT).join("src/data/lorem.txt");
        let mut file = tokio::fs::File::open(&path).await?;
        let mut buf = [0; 64];
        loop {
            let n = file.read(&mut buf [..]).await?;
            if n == 0 { break; }
            tx.send(buf[..n].to_vec()).await.unwrap();
        }
        Ok::<(), std::io::Error>(())
    });
    handles.spawn(async move {
        let id = uuid::Uuid::new_v4();
        let out = std::path::PathBuf::from(PROJECT_ROOT).join(format!("src/data/lorem-{id}.txt"));
        let mut file = tokio::fs::File::create(&out).await?;
        while let Some(chunk) = rx.recv().await {
            file.write_all(&chunk).await?;
        }

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(90)).await;
            tokio::fs::remove_file(&out).await.unwrap()
        });

        Ok(())
    });

    while let Some(handle) = handles.join_next().await {
        handle.unwrap().unwrap();
    }
}
