# MusicWith (Server)
## 개발 환경 준비
- [rustup](https://rustup.rs)
- cargo (nightly)
```sh
rustup toolchain install nightly
rustup default nightly
```
- 프로젝트 디렉토리에 `.env` 파일 생성
```
SPOTIFY_CLIENT_ID=...
SPOTIFY_CLIENT_SECRET=...
SPOTIFY_REDIRECT_SERVER_URI=...
SPOTIFY_REDIRECT_APP_URI=...
BROADCAST_CAPACITY=100
```

## 채팅 테스트 환경 준비
```sh
pip3 install websockets
```

## 실행
```sh
cargo run --release
```

## 채팅 테스트
1. 로컬에서 서버 실행, http://127.0.0.1:8000 에서 열리는 것 확인
2. test/chat_client.py 실행 (여러 인스턴스를 생성해야 테스트할 수 있음)
3. 출력되는 사용법에 따라 사용 가능
