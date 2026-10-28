# Triggerware Python SDK

Async Python SDK for Triggerware.

Install with the `websockets` extra when using cloud/WebSocket connections:

```sh
uv add "triggerware[websockets]"
```

```python
from triggerware import TriggerwareClient

client = await TriggerwareClient.connect_tcp("localhost:5221")

health = await client.system.health()
cursor = await client.cursors.open(sql="select * from my_table")

async for row in cursor:
    print(row)
```

WebSocket connections authorize immediately with the provided API key:

```python
client = await TriggerwareClient.connect_ws("wss://example.com/rpc", "your-api-key-here")
```
