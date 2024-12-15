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
CHAT_HISTORY_DB=...
LYRICS_CACHE_DB=...
SP_DC=...
BROADCAST_CAPACITY=...
```
- `CHAT_HISTORY_DB`: 댓글 기록을 저장할 위치 (예: `chat_history.db`)
- `LYRICS_CACHE_DB`: 가사 캐시를 저장할 위치 (예: `lyrics_cache.db`)
- `SP_DC`: [Spotify](https://spotify.com)의 `sp_dc` 쿠키값 ([참조](https://github.com/akashrchandran/syrics/wiki/Finding-sp_dc))
- `BROADCAST_CAPACITY`: 실시간 댓글 큐 크기 (예: `100`)


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
1. 로컬에서 서버 실행, http://127.0.0.1:8000 에서 열리는 것 확인
2. test/chat_client.py 실행 (여러 인스턴스를 생성해야 테스트할 수 있음)
3. 출력되는 사용법에 따라 사용 가능
