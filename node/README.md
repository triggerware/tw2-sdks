The Triggerware JavaScript SDK is a typed client for calling a Triggerware server from JavaScript or TypeScript. It exposes Triggerware resources such as tables, queries, triggers, agents, tools, and connectors through focused service objects. Use it to avoid building JSON-RPC requests and translating Triggerware's wire-format fields yourself. It also provides convenient cursor, saved-query, and trigger-watching interfaces for working with returned data.

By default, a locally running Triggerware server accepts TCP SDK connections at `localhost:5221`.

1. `new TriggerwareClient(peer: Peer): TriggerwareClient`

   This is used for creating a Triggerware client around an existing JSON-RPC peer.

2. `TriggerwareClient.connectTcp(address: string): Promise<TriggerwareClient>`

   This is used for connecting directly to a Triggerware server at a TCP address. Use `localhost:5221` for the default local server.

3. `TriggerwareClient.connectWs(url: string, apiKey: string): Promise<TriggerwareClient>`

   This is used for connecting to Triggerware over WebSocket and authorizing the connection with an API key.

4. `TriggerwareClient.close(): Promise<void>`

   This is used for closing the client's JSON-RPC connection.

5. `TriggerwareClient.waitClosed(): Promise<void>`

   This is used for waiting until the client's JSON-RPC connection has closed.

6. `TriggerwareClient[Symbol.asyncDispose](): Promise<void>`

   This is used for closing the client when it is disposed through JavaScript's asynchronous disposal protocol.

7. `new ConfigurationService(peer: Peer): ConfigurationService`

   This is used for creating a service that manages Triggerware's instance configuration.

8. `ConfigurationService.get(): Promise<Configuration>`

   This is used for retrieving the current Triggerware configuration without its API key.

9. `ConfigurationService.set(config: ConfigurationUpdate): Promise<Configuration | { url: string }>`

   This is used for updating configuration fields or starting the secure browser-based configuration flow.

10. `ConfigurationService.refresh(): Promise<Configuration>`

    This is used for reloading Triggerware's configuration from disk.

11. `new TablesService(peer: Peer): TablesService`

    This is used for creating a service that manages Triggerware tables.

12. `TablesService.create(creation: TableCreation): Promise<Table>`

    This is used for creating a table from an explicit definition or a natural-language prompt.

13. `TablesService.get(name: string | string[]): Promise<Table | Table[]>`

    This is used for retrieving one or more table definitions.

14. `TablesService.update(name: string, update: TableUpdate): Promise<Table>`

    This is used for updating a standalone table's definition.

15. `TablesService.list(): Promise<string[]>`

    This is used for listing the names of all registered tables.

16. `TablesService.delete(name: string | string[]): Promise<void>`

    This is used for deleting one or more standalone tables.

17. `TablesService.sync(options?: TablesSyncOptions): Promise<void>`

    This is used for reconciling table definitions on disk with the tables registered in Triggerware.

18. `TablesService.configure(name: string, options?: { config?: JsonObject; secure?: boolean }): Promise<AuthorizationResult>`

    This is used for configuring a standalone table directly or through Triggerware's secure configuration flow.

19. `tableMetadataFromString(contents: string): TableMetadata`

    This is used for parsing YAML table metadata into the SDK's camel-cased `TableMetadata` shape.

20. `new CursorsService(peer: Peer): CursorsService`

    This is used for creating a service that opens Triggerware query cursors.

21. `CursorsService.open(options: CursorOpenOptions): Promise<Cursor>`

    This is used for opening a cursor from SQL or a natural-language prompt and receiving its first batch of rows.

22. `new Cursor(peer: Peer, open: CursorOpen): Cursor`

    This is used for representing the state and initial results of an opened Triggerware cursor.

23. `Cursor.fetch(): Promise<Row[]>`

    This is used for fetching the next batch of rows from the cursor.

24. `Cursor.next(): Promise<Row | null>`

    This is used for retrieving the next individual row, fetching another batch when necessary, or returning `null` when exhausted.

25. `Cursor.close(): Promise<void>`

    This is used for closing the server-side cursor and releasing its execution state.

26. `Cursor[Symbol.asyncIterator](): AsyncIterator<Row>`

    This is used for iterating asynchronously over every row produced by the cursor.

27. `new ConnectorsService(peer: Peer): ConnectorsService`

    This is used for creating a service that manages installed Triggerware connectors.

28. `ConnectorsService.install(install: ConnectorInstall): Promise<AuthorizationResult>`

    This is used for installing a catalog or Git-based connector and returning any authorization URL it requires.

29. `ConnectorsService.get(name: string | string[], options?: { tag?: string }): Promise<Connector | Connector[]>`

    This is used for retrieving the configuration and registered resources of one or more installed connectors.

30. `ConnectorsService.list(): Promise<string[]>`

    This is used for listing installed connectors.

31. `ConnectorsService.configure(name: string, options?: { config?: JsonObject; tag?: string; secure?: boolean }): Promise<AuthorizationResult>`

    This is used for configuring an installed connector directly or through the secure configuration flow.

32. `ConnectorsService.authorize(name: string, options?: { tag?: string }): Promise<string>`

    This is used for starting a connector's OAuth authorization flow and retrieving its authorization URL.

33. `ConnectorsService.catalog(): Promise<ConnectorCatalogEntry[]>`

    This is used for retrieving the connectors available in Triggerware's catalog.

34. `ConnectorsService.describe(reference: ConnectorReference): Promise<ConnectorDescription>`

    This is used for inspecting a catalog or Git-based connector before installation.

35. `ConnectorsService.uninstall(name: string | string[], options?: { tag?: string }): Promise<void>`

    This is used for uninstalling one or more connectors and their registered resources.

36. `new AgentsService(peer: Peer): AgentsService`

    This is used for creating a service that manages and runs Triggerware agents.

37. `AgentsService.create(creation: AgentCreation): Promise<Agent>`

    This is used for creating an agent from an explicit definition or a natural-language prompt.

38. `AgentsService.get(name: string | string[]): Promise<Agent | Agent[]>`

    This is used for retrieving one or more agent definitions.

39. `AgentsService.update(name: string, update: AgentUpdate): Promise<Agent>`

    This is used for updating an existing agent's prompt, metadata, or instructions.

40. `AgentsService.list(): Promise<string[]>`

    This is used for listing all agent names.

41. `AgentsService.delete(name: string | string[]): Promise<void>`

    This is used for deleting one or more agents.

42. `AgentsService.run(name: string, data: JsonObject): Promise<JsonObject>`

    This is used for running an agent manually with a JSON object as its input.

43. `AgentsService.sync(options?: { agents?: string[]; connectors?: string[] }): Promise<void>`

    This is used for reconciling agent definitions on disk with agents registered in Triggerware.

44. `new ToolsService(peer: Peer): ToolsService`

    This is used for creating a service that manages Triggerware agent tools.

45. `ToolsService.create(creation: ToolCreation): Promise<Tool>`

    This is used for creating a standalone tool from an explicit definition or a natural-language prompt.

46. `ToolsService.get(name: string | string[]): Promise<Tool | Tool[]>`

    This is used for retrieving one or more tool definitions.

47. `ToolsService.update(name: string, update: ToolUpdate): Promise<Tool>`

    This is used for updating a standalone tool's definition.

48. `ToolsService.list(): Promise<string[]>`

    This is used for listing all registered tool names.

49. `ToolsService.delete(name: string | string[]): Promise<void>`

    This is used for deleting one or more standalone tools.

50. `ToolsService.sync(options?: ToolsSyncOptions): Promise<void>`

    This is used for reconciling tool definitions on disk with tools registered in Triggerware.

51. `ToolsService.configure(name: string, options?: { config?: JsonObject; secure?: boolean }): Promise<AuthorizationResult>`

    This is used for configuring a standalone tool directly or through Triggerware's secure configuration flow.

52. `toolMetadataFromString(contents: string): ToolMetadata`

    This is used for parsing YAML tool metadata into the SDK's camel-cased `ToolMetadata` shape.

53. `new LoggingService(peer: Peer): LoggingService`

    This is used for creating a service that controls Triggerware's runtime log level.

54. `LoggingService.set(level: LogLevel): Promise<void>`

    This is used for setting the minimum level written to Triggerware's operational log.

55. `new McpService(peer: Peer): McpService`

    This is used for creating a service that forwards MCP operations through Triggerware.

56. `McpService.call<T = unknown>(method: string, params?: JsonObject): Promise<T>`

    This is used for calling a method on Triggerware's built-in MCP endpoint.

57. `new QueriesService(peer: Peer): QueriesService`

    This is used for creating a service that manages saved Triggerware queries.

58. `QueriesService.create(creation: QueryCreation): Promise<Query>`

    This is used for creating a saved query from SQL or a natural-language prompt.

59. `QueriesService.convert(conversion: QueryConversion): Promise<QueryFetch>`

    This is used for converting a saved query into a trigger and returning its initial baseline rows.

60. `QueriesService.get(name: string | string[]): Promise<Query | Query[]>`

    This is used for retrieving one or more saved queries as stateful query objects.

61. `QueriesService.list(): Promise<string[]>`

    This is used for listing all saved query names.

62. `QueriesService.delete(name: string | string[]): Promise<void>`

    This is used for deleting one or more saved queries.

63. `new Query(peer: Peer, definition: QueryDefinition): Query`

    This is used for representing a saved query and its local iteration state.

64. `Query.reset(): Promise<void>`

    This is used for resetting the saved query's server-side cursor and refreshing its definition.

65. `Query.fetch(options?: QueryFetchOptions): Promise<Row[]>`

    This is used for fetching the next batch of rows from a saved query.

66. `Query.next(options?: QueryFetchOptions): Promise<Row | null>`

    This is used for retrieving the next individual saved-query row or returning `null` when exhausted.

67. `Query[Symbol.asyncIterator](): AsyncIterator<Row>`

    This is used for iterating asynchronously over rows from the saved query.

68. `new RuntimesService(peer: Peer): RuntimesService`

    This is used for creating a service that manages Triggerware's external runtimes.

69. `RuntimesService.list(): Promise<string[]>`

    This is used for listing installed external runtimes.

70. `RuntimesService.install(name: ExternalRuntime): Promise<void>`

    This is used for installing the selected Python or Node runtime.

71. `RuntimesService.uninstall(name: ExternalRuntime): Promise<void>`

    This is used for uninstalling the selected external runtime.

72. `new SystemService(peer: Peer): SystemService`

    This is used for creating a service that accesses Triggerware system and connection operations.

73. `SystemService.health(): Promise<Health>`

    This is used for checking whether the Triggerware server is healthy enough to accept requests.

74. `SystemService.update(): Promise<SystemUpdate>`

    This is used for updating the local Triggerware installation and reporting the updated components and restart requirement.

75. `SystemService.authorize(apiKey: string): Promise<boolean>`

    This is used for authorizing a WebSocket connection with a Triggerware API key.

76. `SystemService.authorized(): Promise<boolean>`

    This is used for checking whether the current connection is authorized.

77. `new TriggersService(peer: Peer): TriggersService`

    This is used for creating a service that manages and watches Triggerware triggers.

78. `TriggersService.create(creation: TriggerCreation): Promise<Trigger>`

    This is used for creating a trigger from an explicit query definition or a natural-language prompt.

79. `TriggersService.get(name: string | string[]): Promise<Trigger | Trigger[]>`

    This is used for retrieving one or more trigger definitions.

80. `TriggersService.update(name: string, update: TriggerUpdate): Promise<Trigger>`

    This is used for updating an existing trigger's query, schedule, execution settings, or enabled state.

81. `TriggersService.list(): Promise<string[]>`

    This is used for listing all registered trigger names.

82. `TriggersService.delete(name: string | string[]): Promise<void>`

    This is used for deleting one or more triggers.

83. `TriggersService.enable(name: string): Promise<void>`

    This is used for enabling scheduled execution of a trigger.

84. `TriggersService.disable(name: string): Promise<void>`

    This is used for disabling scheduled execution of a trigger.

85. `TriggersService.run(name: string, options?: RunOptions): Promise<TriggerDelta>`

    This is used for running a trigger immediately and returning its added and deleted rows.

86. `TriggersService.poll(name: string): Promise<TriggerDelta[]>`

    This is used for draining and returning the trigger's cached deltas.

87. `TriggersService.watch(name: string, callback: TriggerCallback, options?: { resumeWindowMs?: number }): Promise<string>`

    This is used for subscribing to a trigger's retained and future deltas and receiving a resume token.

88. `TriggersService.resume(name: string, resumeToken: string, callback: TriggerCallback, options?: { resumeWindowMs?: number }): Promise<string>`

    This is used for resuming a trigger subscription from a valid token and receiving the active token.

89. `TriggersService.unwatch(options: { name?: string; resumeToken?: string }): Promise<void>`

    This is used for stopping a trigger subscription by trigger name or resume token.
