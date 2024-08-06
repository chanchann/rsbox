import websocket
import time
import threading

def on_message(ws, message):
    if isinstance(message, bytes):
        print({
            "messageLength": len(message),
            "messageType": "binary",
            "messageData": message
        })
    else:
        print({
            "messageLength": len(message),
            "messageType": "text",
            "messageData": message
        })

def on_open(ws):
    def run():
        count = 0
        while True:
            count += 1
            ws.send(f"send message count: {count}")
            time.sleep(1)
    
    threading.Thread(target=run).start()

if __name__ == "__main__":
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp("ws://localhost:8080/",
                                on_message=on_message,
                                on_open=on_open)
    ws.run_forever()