# Triggerware JavaScript SDK

Modern TypeScript SDK for Triggerware.

```ts
import { TriggerwareClient } from "triggerware"

const client = await TriggerwareClient.connectTcp("localhost:5221")

const health = await client.system.health()
const cursor = await client.cursors.open({ sql: "select * from my_connector" })

for await (const row of cursor) {
  console.log(row)
}
```

WebSocket connections authorize immediately with the provided API key:

```ts
const client = await TriggerwareClient.connectWs(
  "wss://example.com/rpc",
  "your-api-key-here",
)
```
