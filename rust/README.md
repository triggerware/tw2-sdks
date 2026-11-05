The Triggerware Rust SDK is a typed client for communicating with a Triggerware server. It provides Rust methods for every supported Triggerware resource, including tables, queries, triggers, agents, tools, connectors, and runtimes. Use it to make JSON-RPC calls without manually constructing requests or decoding responses. It supports direct TCP connections and authenticated HTTP connections.

By default, a locally running Triggerware server accepts TCP SDK connections at `localhost:5221`.

1. `pub async fn TriggerwareClient::connect_tcp(addr: impl tokio::net::ToSocketAddrs) -> Result<TriggerwareClient>`

   This is used for connecting to a Triggerware server over TCP and creating a client. Use `localhost:5221` for the default local server.

2. `pub async fn TriggerwareClient::connect_http(url: &str, bearer_token: impl Into<String>) -> Result<TriggerwareClient>`

   This is used for connecting to a Triggerware HTTP endpoint with a bearer token and creating a client.

3. `pub fn TriggerwareClient::new(peer: Peer) -> TriggerwareClient`

   This is used for creating a Triggerware client from an existing JSON-RPC peer.

4. `pub async fn ConfigurationService::get(&self) -> Result<Configuration>`

   This is used for retrieving the server's current configuration.

5. `pub async fn ConfigurationService::set(&self, options: ConfigurationUpdate) -> Result<ConfigurationResult>`

   This is used for updating configuration fields or starting secure browser-based configuration.

6. `pub async fn ConfigurationService::refresh(&self) -> Result<Configuration>`

   This is used for reloading the server configuration from disk.

7. `fn TableMetadata::from_str(contents: &str) -> Result<TableMetadata>`

   This is used for parsing table metadata from YAML or JSON text.

8. `pub async fn TablesService::create(&self, options: TableWrite) -> Result<Table>`

   This is used for creating a table from explicit fields or a natural-language prompt.

9. `pub async fn TablesService::update(&self, options: TableWrite) -> Result<Table>`

   This is used for updating an existing standalone table.

10. `pub async fn TablesService::get(&self, names: Vec<String>) -> Result<Vec<Table>>`

    This is used for retrieving one or more tables by name.

11. `pub async fn TablesService::delete(&self, names: Vec<String>) -> Result<()>`

    This is used for deleting one or more standalone tables.

12. `pub async fn TablesService::list(&self) -> Result<Vec<String>>`

    This is used for listing all registered table names.

13. `pub async fn TablesService::sync(&self, options: TablesSyncOptions) -> Result<()>`

    This is used for reconciling table definitions on disk with the tables registered in Triggerware.

14. `pub async fn TablesService::configure(&self, options: ResourceConfiguration) -> Result<AuthorizationResult>`

    This is used for configuring a standalone table or starting its secure configuration flow.

15. `pub async fn CursorsService::open(&self, options: CursorOpenOptions) -> Result<Cursor>`

    This is used for opening a cursor from SQL or a natural-language prompt and receiving its first rows.

16. `pub fn Cursor::sql(&self) -> &str`

    This is used for reading the SQL associated with a cursor.

17. `pub fn Cursor::signature(&self) -> &[CursorColumn]`

    This is used for reading the cursor's result-column signature.

18. `pub fn Cursor::is_exhausted(&self) -> bool`

    This is used for checking whether a cursor has no more rows to fetch.

19. `pub async fn Cursor::fetch(&mut self) -> Result<Vec<Vec<Value>>>`

    This is used for fetching the next batch of rows from a cursor.

20. `pub async fn Cursor::next(&mut self) -> Result<Option<Vec<Value>>>`

    This is used for consuming one row at a time while fetching additional batches as needed.

21. `pub async fn Cursor::close(&mut self) -> Result<()>`

    This is used for closing a cursor and releasing its server-side execution state.

22. `pub async fn ConnectorsService::install(&self, options: ConnectorInstall) -> Result<AuthorizationResult>`

    This is used for installing a connector from the catalog or a repository URL.

23. `pub async fn ConnectorsService::get(&self, names: Vec<String>, tag: Option<String>) -> Result<Vec<Connector>>`

    This is used for retrieving installed connectors and their resources.

24. `pub async fn ConnectorsService::list(&self) -> Result<Vec<String>>`

    This is used for listing all installed connectors.

25. `pub async fn ConnectorsService::uninstall(&self, names: Vec<String>, tag: Option<String>) -> Result<()>`

    This is used for uninstalling one or more connectors and their installed resources.

26. `pub async fn ConnectorsService::configure(&self, options: ConnectorConfiguration) -> Result<AuthorizationResult>`

    This is used for configuring an installed connector or starting its secure configuration flow.

27. `pub async fn ConnectorsService::authorize(&self, name: impl Into<String>, tag: Option<String>) -> Result<String>`

    This is used for starting OAuth authorization for a connector and returning its authorization URL.

28. `pub async fn ConnectorsService::catalog(&self) -> Result<Vec<ConnectorCatalogEntry>>`

    This is used for retrieving the maintained connector catalog.

29. `pub async fn ConnectorsService::describe(&self, reference: ConnectorReference) -> Result<ConnectorDescription>`

    This is used for inspecting a catalog connector or repository connector before installing it.

30. `pub async fn LoggingService::set(&self, level: LogLevel) -> Result<()>`

    This is used for setting the minimum runtime log level.

31. `pub async fn AgentsService::create(&self, options: AgentWrite) -> Result<Agent>`

    This is used for creating an agent from explicit fields or a natural-language prompt.

32. `pub async fn AgentsService::update(&self, options: AgentWrite) -> Result<Agent>`

    This is used for updating an existing agent.

33. `pub async fn AgentsService::get(&self, names: Vec<String>) -> Result<Vec<Agent>>`

    This is used for retrieving one or more agents by name.

34. `pub async fn AgentsService::delete(&self, names: Vec<String>) -> Result<()>`

    This is used for deleting one or more agents.

35. `pub async fn AgentsService::list(&self) -> Result<Vec<String>>`

    This is used for listing all registered agent names.

36. `pub async fn AgentsService::sync(&self, agents: Option<Vec<String>>, connectors: Option<Vec<String>>) -> Result<()>`

    This is used for reconciling standalone or connector agent definitions from disk.

37. `pub async fn AgentsService::run(&self, name: impl Into<String>, data: Map<String, Value>) -> Result<Map<String, Value>>`

    This is used for running an agent manually with structured input data.

38. `pub async fn ToolsService::create(&self, options: ToolWrite) -> Result<Tool>`

    This is used for creating a standalone tool from explicit fields or a natural-language prompt.

39. `pub async fn ToolsService::update(&self, options: ToolWrite) -> Result<Tool>`

    This is used for updating an existing standalone tool.

40. `pub async fn ToolsService::get(&self, names: Vec<String>) -> Result<Vec<Tool>>`

    This is used for retrieving one or more tools by name.

41. `pub async fn ToolsService::delete(&self, names: Vec<String>) -> Result<()>`

    This is used for deleting one or more standalone tools.

42. `pub async fn ToolsService::list(&self) -> Result<Vec<String>>`

    This is used for listing all registered tool names.

43. `pub async fn ToolsService::sync(&self, tools: Option<Vec<String>>, connectors: Option<Vec<String>>) -> Result<()>`

    This is used for reconciling standalone or connector tool definitions from disk.

44. `pub async fn ToolsService::configure(&self, options: ResourceConfiguration) -> Result<AuthorizationResult>`

    This is used for configuring a standalone tool or starting its secure configuration flow.

45. `pub async fn McpService::call(&self, method: impl Into<String>, params: Option<Map<String, Value>>) -> Result<Value>`

    This is used for forwarding an MCP method and parameters through Triggerware's MCP endpoint.

46. `pub async fn QueriesService::create(&self, options: QueryWrite) -> Result<Query>`

    This is used for creating a saved query from SQL or a natural-language prompt.

47. `pub async fn QueriesService::convert(&self, options: QueryConvertOptions) -> Result<QueryConvertResult>`

    This is used for converting a saved query into a trigger and returning its initial rows and exhaustion state.

48. `pub async fn QueriesService::get(&self, names: Vec<String>) -> Result<Vec<Query>>`

    This is used for retrieving one or more saved queries by name.

49. `pub async fn QueriesService::delete(&self, names: Vec<String>) -> Result<()>`

    This is used for deleting one or more saved queries.

50. `pub async fn QueriesService::list(&self) -> Result<Vec<String>>`

    This is used for listing all saved query names.

51. `pub fn Query::name(&self) -> &str`

    This is used for reading a saved query's name.

52. `pub fn Query::description(&self) -> &str`

    This is used for reading a saved query's description.

53. `pub fn Query::params(&self) -> Option<&[QuerySignature]>`

    This is used for reading the parameters accepted by a saved query.

54. `pub fn Query::sql(&self) -> Option<&str>`

    This is used for reading a saved query's SQL text when present.

55. `pub fn Query::prompt(&self) -> Option<&str>`

    This is used for reading the prompt that produced a saved query when present.

56. `pub fn Query::batch_size(&self) -> Option<usize>`

    This is used for reading a saved query's configured batch size.

57. `pub fn Query::timeout_ms(&self) -> Option<u64>`

    This is used for reading a saved query's configured timeout.

58. `pub fn Query::signature(&self) -> &[QuerySignature]`

    This is used for reading a saved query's result-column signature.

59. `pub fn Query::is_exhausted(&self) -> bool`

    This is used for checking whether a saved query has no more rows to fetch.

60. `pub async fn Query::reset(&mut self) -> Result<()>`

    This is used for resetting a saved query's persistent cursor state.

61. `pub async fn Query::fetch(&mut self, options: QueryFetchOptions) -> Result<Vec<Vec<Value>>>`

    This is used for fetching a batch of rows from a saved query with optional parameters and row limit.

62. `pub async fn Query::next(&mut self) -> Result<Option<Vec<Value>>>`

    This is used for consuming one saved-query row at a time while fetching additional batches as needed.

63. `pub async fn RuntimesService::list(&self) -> Result<Vec<ExternalRuntime>>`

    This is used for listing installed external runtimes.

64. `pub async fn RuntimesService::install(&self, name: ExternalRuntime) -> Result<()>`

    This is used for installing a Python or Node runtime.

65. `pub async fn RuntimesService::uninstall(&self, name: ExternalRuntime) -> Result<()>`

    This is used for uninstalling an external runtime.

66. `pub async fn SystemService::health(&self) -> Result<Health>`

    This is used for checking whether the Triggerware server is healthy.

67. `pub async fn SystemService::update(&self) -> Result<SystemUpdate>`

    This is used for updating the local Triggerware installation and reporting what changed.

68. `pub async fn SystemService::authorize(&self, api_key: impl Into<String>) -> Result<bool>`

    This is used for authorizing the current connection with an API key.

69. `pub async fn SystemService::authorize_jwt(&self, jwt: impl Into<String>) -> Result<bool>`

    This is used for authorizing the current connection with a JSON Web Token.

70. `pub async fn SystemService::authorized(&self) -> Result<bool>`

    This is used for checking whether the current connection is authorized.

71. `pub async fn TriggersService::create(&self, options: TriggerWrite) -> Result<Trigger>`

    This is used for creating a trigger from SQL or a natural-language prompt.

72. `pub async fn TriggersService::update(&self, options: TriggerWrite) -> Result<Trigger>`

    This is used for updating an existing trigger.

73. `pub async fn TriggersService::get(&self, names: Vec<String>) -> Result<Vec<Trigger>>`

    This is used for retrieving one or more triggers by name.

74. `pub async fn TriggersService::delete(&self, names: Vec<String>) -> Result<()>`

    This is used for deleting one or more triggers.

75. `pub async fn TriggersService::list(&self) -> Result<Vec<String>>`

    This is used for listing all registered trigger names.

76. `pub async fn TriggersService::enable(&self, name: impl Into<String>) -> Result<()>`

    This is used for enabling scheduled execution for a trigger.

77. `pub async fn TriggersService::disable(&self, name: impl Into<String>) -> Result<()>`

    This is used for disabling scheduled execution for a trigger.

78. `pub async fn TriggersService::run(&self, name: impl Into<String>, options: RunOptions) -> Result<TriggerDelta>`

    This is used for running a trigger immediately and returning its added and deleted rows.

79. `pub async fn TriggersService::poll(&self, name: impl Into<String>) -> Result<Vec<TriggerDelta>>`

    This is used for draining the retained deltas for a trigger.

80. `pub async fn TriggersService::watch<F>(&self, name: impl Into<String>, resume_window_ms: Option<u64>, callback: F) -> Result<String> where F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static`

    This is used for subscribing to retained and future trigger deltas over a streaming connection.

81. `pub async fn TriggersService::resume<F>(&self, name: impl Into<String>, resume_token: impl Into<String>, resume_window_ms: Option<u64>, callback: F) -> Result<String> where F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static`

    This is used for resuming a trigger subscription from a valid resume token.

82. `pub async fn TriggersService::unwatch(&self, name: Option<String>, resume_token: Option<String>) -> Result<()>`

    This is used for stopping a trigger subscription by name or resume token.
