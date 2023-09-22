# 拼多多开发平台Rust SDK

拼多多开发平台sdk，支持消息推送

## Examples

### 设置环境变量

```bash
set PDD_CLIENT_ID=you_client_id
set PDD_CLIENT_SECRET=you_secret

#不是必须
set PDD_ACCESS_TOKEN=you_access_token
```

### 请求订单列表

```rust
    let client = Client::from_env()?;
    let req = PddOrderListGet {
        start_confirm_at: Some(1689250974),
        end_confirm_at: Some(1689260974),
        use_has_next: Some(true),
        order_status: Some(1),
        ..Default::default()
    };
    client.send(req).await?;
```

### 消息推送

需要开启`pmc-native-tls`feature

```rust
    let client = PmcClient::new(Config::from_env()?);
    let mut s = client.connect().await?;

    while let Some(msg) = s.next().await {
        let msg = msg.unwrap();
        println!("{:?}", msg);
        
        //ack消息
        s.ack(&msg).await;
    }
```

### 图片上传

```rust
    let client = Client::from_env()?;
    let req = PddGoodsFilespaceImageUpload {
        file: Some(PddFile::from_file("examples/1.jpg").await?),
    };
    let rsp = client.file_upload(req).await?;
```
