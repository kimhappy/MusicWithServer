# MusicWith (Server)
## Development Requirements
- [rustup](https://rustup.rs/)
- cargo (nightly)
```sh
rustup toolchain install nightly
rustup default nightly
```
- Create `.env` file
```
SPOTIFY_CLIENT_ID=...
SPOTIFY_CLIENT_SECRET=...
SPOTIFY_REDIRECT_SERVER_URI=...
SPOTIFY_REDIRECT_APP_URI=...
```

## Run
```sh
cargo run --release
```