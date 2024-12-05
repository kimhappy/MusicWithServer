# MusicWith (Server)
## 개발 환경 준비
- [rustup](https://rustup.rs)
- cargo (nightly)
```sh
rustup toolchain install nightly
rustup default nightly
```
- `ssh` 디렉토리에 `key.pem` (개인키), `certs.pem` (자체 서명 인증서) 생성
```sh
brew install openssl
openssl genrsa -out key.pem 2048 # key.pem
openssl req -new -x509 -key key.pem -out certs.pem -days 365 # certs.pem
```
- 프로젝트 디렉토리에 `.env` 파일 생성
```
SPOTIFY_CLIENT_ID=...
SPOTIFY_CLIENT_SECRET=...
SPOTIFY_REDIRECT_SERVER_URI=...
SPOTIFY_REDIRECT_APP_URI=...
SP_DC=...
BROADCAST_CAPACITY=...
CHAT_HISTORY_DB=...
```
- `SPOTIFY_CLIENT_ID`, `SPOTIFY_CLIENT_SECRET`: [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)에서 발급
- `SPOTIFY_REDIRECT_SERVER_URI`: 서버의 `callback` endpoint (예: `http://localhost:8000/callback`)
- `SPOTIFY_REDIRECT_APP_URI`: 앱의 `callback` endpoint (예: `com.kimhappy.musicwith://callback`)
- `SP_DC`: [Spotify](https://spotify.com)의 `sp_dc` 쿠키값 ([참조](https://github.com/akashrchandran/syrics/wiki/Finding-sp_dc))
- `BROADCAST_CAPACITY`: 실시간 댓글 큐 크기 (예: `100`)
- `CHAT_HISTORY_DB`: 댓글 기록을 저장할 위치 (예: `chat_history.db`)

## 실행
```sh
cargo run --release
```

## 테스트
### 채팅
```sh
pip3 install websockets
python3 test/chat_client.py
```
1. 로컬에서 서버 실행, https://127.0.0.1:8000 에서 열리는 것 확인
2. test/chat_client.py 실행 (여러 인스턴스를 생성해야 테스트할 수 있음)
3. 출력되는 사용법에 따라 사용 가능
