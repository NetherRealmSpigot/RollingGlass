use std::io::{stdout, Write};

use rolling_glass::{DEFAULT_PORT, LATEST, MINECRAFT_1_7, MINECRAFT_1_8, ping};

#[tokio::test]
async fn test_ping() {
    assert!(ping(&String::new(), DEFAULT_PORT, &String::new(), MINECRAFT_1_8, 3).await.is_err());
    assert!(ping(&String::from("::1"), DEFAULT_PORT, &String::new(), MINECRAFT_1_8, 3).await.is_err());
    assert!(ping(&String::from("127.0.0.1"), DEFAULT_PORT, &String::new(), MINECRAFT_1_7 - 1, 3).await.is_err());
    assert!(ping(&String::from("127.0.0.1"), DEFAULT_PORT, &String::new(), LATEST + 1, 3).await.is_err());
    assert!(ping(&String::from("127.0.0.1"), DEFAULT_PORT, &String::new(), LATEST + 1, 0).await.is_err());
    assert!(ping(&String::from("doesntexist.local"), DEFAULT_PORT, &String::new(), LATEST + 1, 0).await.is_err());
    assert!(ping(&String::from("127.0.0.1"), DEFAULT_PORT, &String::new(), MINECRAFT_1_8, 1).await.is_err());
    assert!(ping(&String::from("mc.hypixel.net"), DEFAULT_PORT, &String::from("hypixel.gg"), LATEST, 3).await.is_err());

    let res = ping(
        &String::from("play.cubecraft.net"),
        DEFAULT_PORT,
        &String::from("cubecraft.gg"),
        LATEST,
    3).await;
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(!res.is_empty());
    let _ = stdout().write_all(&res);
}
