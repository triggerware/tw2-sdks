All services having to do with `logs` currently are not supported at all. They're not even implemented in code.

Every SDK uses the appropriate JSON-RPC library located at `../tw2-jsonrpc/*`. So Rust uses `tw2-jsonrpc`, Python uses `tw2-jsonrpc`, etc.

This doc uses snake_case for most things, but other case forms should be used when the language expects them.

Everything should be async if possible, since my JSON-RPC libraries are all async if the language supports it.

Everything should use the JSON-RPC errors / exceptions from the appropriate JSON-RPC library. Do not invent a custom Triggerware SDK error type.

SDK objects should generally feel immutable from the user's perspective. This means local fields on things like `TriggerwareClient`, `Connector`, `Cursor`, `Query`, and `Trigger` should not be public mutable fields and should not be casually reassigned by SDK users. Use accessors/getters for local fields where the language expects them. Methods may still perform remote operations like update, delete, reset, enable, disable, etc. If a remote operation changes the resource's definition, the object may refresh itself internally.

Services should usually be for creating, loading, listing, syncing, and opening resources. Once an SDK has returned a resource object, lifecycle methods that clearly belong to that object should live on the object too. For example, prefer `connector.update(...)`, `query.fetch()`, and `trigger.run()` over forcing users back through `client.connectors.update(...)`, `client.queries.fetch(...)`, or `client.triggers.run(...)`. Dynamic languages may still provide service-level convenience overloads, but the object-owned API is the primary shape.

## The TriggerwareClient

Each SDK should expose an object (class? struct) called `TriggerwareClient`.

It can connect via WebSockets or TCP.

The WebSocket version takes an API key. It will eventually connect to `api.triggerware.ai/instances`, hardcoded for now. This won't fully work yet and that's fine.

The TCP version takes a TCP address string. Use the plain `host:port` form.

```text
# either
client = await TriggerwareClient.connect_ws("your-api-key-here")

# or
client = await TriggerwareClient.connect_tcp("localhost:5000")
```

The client always has these service fields:

- `configuration`
- `connectors`
- `cursors`
- `extensions`
- `prompts`
- `queries`
- `system`
- `triggers`

### configuration

The client always has a field called `configuration` with get, set, and refresh methods that match the JSON-RPC spec.

This maps to the `configuration.*` JSON-RPC methods.

### connectors

The client always has a field called `connectors` with create, get, list, and sync methods.

`connectors.get` returns a Connector class / struct.

`connectors.create` should return a Connector too.

`connectors.list` returns connector names, matching the JSON-RPC docs.

Update, delete, and configure should live on Connector objects:

```text
connector = await client.connectors.get("stripe_charges")
await connector.update(...)
await connector.configure(...)
await connector.delete()
```

### cursors

The client always has a field called `cursors` with an open method.

`cursors.open` returns a Cursor class / struct.

Cursor objects own their fetch, next/iteration, and close behavior.

### extensions

The client has a field called `extensions` with install, get, list, configure, catalog, describe, and uninstall methods that match the spec.

### prompts

The client has a field called `prompts` with get, set, reset, and sync methods that match the spec.

### queries

The client has a field called `queries` with create, get, and list methods.

`queries.get` returns a Query class / struct.

`queries.create` should return a Query too.

`queries.list` returns query names, matching the JSON-RPC docs.

Query objects own update, reset, delete, fetch, and next/iteration:

```text
query = await client.queries.get("recent_charges")
await query.fetch()
await query.next()
await query.reset()
await query.update(...)
await query.delete()
```

Saved queries do not have close because they are not owned by a single connection.

Query fetch should use the batch size and timeout stored on the saved query. Do not require timeout or batch size arguments on every `query.fetch()` or `query.next()` call.

### system

The client has a field called `system` with health, authorize, and authorized methods that match the JSON-RPC docs.

For WebSockets in particular, authorization is called right away when someone does `connect_ws` with the API key, so users do not necessarily have to call authorize themselves.

### triggers

The client has a field called `triggers` with create, get, and list methods.

`triggers.get` returns a Trigger class / struct.

`triggers.create` should return a Trigger too.

Trigger objects own run, poll, enable, disable, update, delete, and watch:

```text
trigger = await client.triggers.get("new_refunds")
await trigger.run()
await trigger.poll()
await trigger.enable()
await trigger.disable()
await trigger.update(...)
await trigger.delete()
await trigger.watch(callback)
await trigger.unwatch()
```

Dynamic languages may add service-level overloads that accept either trigger names or Trigger objects, but the object-owned API should be the main path.

For watch and unwatch, the SDK supplies a callback. The callback takes `added` and `deleted` params. When a JSON-RPC notification arrives, the SDK must destructure every delta and call the callback with that delta's added and deleted rows.

```text
callback(added, deleted)
```

Only one watch per trigger per client/peer is supported. A Trigger should store its watch state and resume token privately. `trigger.watch(...)` sets that state after calling `triggers.watch` and returns the resume token; `trigger.resume(...)` resumes from an existing token; `trigger.unwatch()` uses the stored resume token, then clears local watch state.

## The Connector

A Connector is readonly and mostly just stores what `connectors.get` returned.

A Connector should know its name.

A Connector has update, configure, and delete methods:

```text
await connector.update(...)
await connector.configure(...)
await connector.delete()
```

These call `connectors.update`, `connectors.configure`, and `connectors.delete` through the client/peer that created the Connector. If update changes the connector definition, the Connector should replace its cached fields from the `connectors.update` response.

## The Cursor

A Cursor class / struct stores its cursor id and enough client reference/context to fetch and close itself.

In Rust, storing a shared JSON-RPC peer/client handle on the resource is fine. Do not force users to pass the client back into object methods.

A cursor has its own fetch and close methods, meaning you can create one and use it how you'd expect without having to do `client.cursors.fetch(cursor)`.

```text
await cursor.fetch()
await cursor.close()
```

A Cursor should support iteration if the language supports it.

Iteration yields one row at a time. Internally, the iterator uses the current batch of rows first. Once those rows are exhausted, it calls fetch again, uses that batch, and keeps doing that until the returned result says it is exhausted.

## The Query

A Query stores its name and any config fields returned by `queries.get`, `queries.create`, or `queries.update`.

A Query can fetch, iterate one row at a time, and reset itself:

```text
await query.fetch()
await query.next()
await query.reset()
```

A Query can also update or delete itself:

```text
await query.update(...)
await query.delete()
```

`query.fetch()` and `query.next()` use the saved query's stored batch size and timeout. Per-call timeout and batch size parameters are not part of the normal object API. Cursors, queries, and trigger runs should share a common run-options object for batch size and timeout where the language benefits from that.

Saved queries do not have close because they are not owned by a single connection.

## The Trigger

A Trigger stores its name, prompt/sql, signature, and grouped options returned by `triggers.get`, `triggers.create`, or `triggers.update`.

A Trigger can run, poll, enable, disable, update, and delete itself:

```text
await trigger.run()
await trigger.poll()
await trigger.enable()
await trigger.disable()
await trigger.update(...)
await trigger.delete()
```

A Trigger can also watch itself:

```text
await trigger.watch(callback)
await trigger.resume(resume_token, callback)
await trigger.unwatch()
```
