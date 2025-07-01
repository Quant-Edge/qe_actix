# qe-actix

Due to certain reasons, some location are unable to communicate directly with Binance. Deploy this project on a server that can communicate with both side.

## Configuration
Add `keys.toml` file in `.../qe_actix/keys.toml`, content like:
```toml
[binance1]
apiKey = 'xxxx'
secret = 'xxxx'

[sub1]
apiKey = "xxxx"
secret = "xxxx"

[sub2]
apiKey = "xxxx"
secret = "xxxx"
```