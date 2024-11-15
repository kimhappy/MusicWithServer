import asyncio
import websockets
import json

async def chat_client():
    while True:
        user_id = input('user_id: ')

        while True:
            track_id = input('track_id: ')
            uri      = f'ws://127.0.0.1:8000/chat/{track_id}/{user_id}'

            try:
                async with websockets.connect(uri) as websocket:
                    await websocket.send(json.dumps({ 'History': {} }))
                    await websocket.send(json.dumps({ 'Online' : {} }))

                    send_task     = asyncio.create_task(send_message(websocket, user_id))
                    receive_task  = asyncio.create_task(receive_message(websocket))
                    done, pending = await asyncio.wait(
                        [send_task, receive_task],
                        return_when = asyncio.FIRST_COMPLETED)

                    for task in pending:
                        task.cancel()

            except Exception as e:
                print(f'서버 연결 실패: {e}')
                continue

async def send_message(websocket, user_id):
    while True:
        message = await asyncio.get_event_loop().run_in_executor(None, input)

        if message == 'q':
            print('연결 종료')
            await websocket.close()
            break

        elif message.startswith('@'):
            try:
                parts    = message.split(' ', 1)
                reply_to = int(parts[ 0 ][ 1: ])
                content  = parts[ 1 ]
                msg      = {
                    'Chat': {
                        'content' : content,
                        'reply_to': reply_to
                    }
                }

                await websocket.send(json.dumps(msg))

            except Exception as e:
                print(f'잘못된 형식: {e}')

        elif message.startswith('!delete '):
            try:
                chat_id = int(message.split(' ')[ 1 ])
                msg     = {
                    'Delete': {
                        'chat_id': chat_id
                    }
                }

                await websocket.send(json.dumps(msg))

            except Exception as e:
                print(f'잘못된 형식: {e}')

        else:
            parts = message.split(' ', 1)
            msg   = {
                'Chat': {
                    'time'   : int(parts[ 0 ]),
                    'content': parts[ 1 ]
                }
            }

            await websocket.send(json.dumps(msg))

async def receive_message(websocket):
    def print_chat(chat):
        user_id  = chat.get('user_id' )
        content  = chat.get('content' )
        chat_id  = chat.get('chat_id' )
        reply_to = chat.get('reply_to')

        if content is not None:
            if reply_to is not None:
                print(f'[{chat_id}] ([{reply_to}]에 답장) {user_id}: {content}')
            else:
                time = chat.get('time')
                print(f'[{chat_id}] ({time}) {user_id}: {content}')
        else:
            print(f'[{chat_id}] 삭제됨')

    while True:
        try:
            message = await websocket.recv()
            data    = json.loads(message)

            if 'Chat' in data:
                print_chat(data['Chat'])

            elif 'Delete' in data:
                delete  = data['Delete']
                chat_id = delete.get('chat_id')
                print(f'[{chat_id}] 삭제됨')

            elif 'Online' in data:
                online = data['Online']
                items   = online.get('items', [])

                print('=== 현재 접속자 시작 ===')

                for item in items:
                    print(item)

                print('=== 현재 접속자 끝 ===')

            elif 'History' in data:
                history = data['History']
                items   = history.get('items', [])

                print('=== 이전 메시지 시작 ===')

                for item in items:
                    print_chat(item)

                print('=== 이전 메시지 끝 ===')

            elif 'Join' in data:
                join    = data['Join']
                user_id = join.get('user_id')
                print(f'{user_id}님이 입장하였습니다.')

            elif 'Leave' in data:
                leave   = data['Leave']
                user_id = leave.get('user_id')
                print(f'{user_id}님이 퇴장하였습니다.')

        except websockets.ConnectionClosed:
            print('연결 종료')
            break

        except Exception as e:
            print(f'메시지 수신 오류: {e}')

if __name__ == '__main__':
    print('사용법:')
    print('1. user_id를 입력합니다.')
    print('2. track_id를 입력합니다.')
    print('3. <time> <message>로 댓글을 달 수 있습니다 (time은 정수).')
    print('4. @<chat_id> <message>로 대댓글을 달 수 있습니다.')
    print('5. !delete <chat_id>로 댓글을 삭제할 수 있습니다.')
    print('6. q를 입력하면 연결이 종료됩니다.')
    asyncio.run(chat_client())
