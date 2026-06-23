from __future__ import annotations

from typing import Any, NotRequired, TypedDict

import pysonrpc as rpc


class QuerySignature(TypedDict):
    name: str
    type: str


class QueryDefinition(TypedDict):
    name: str
    sql: NotRequired[str]
    prompt: NotRequired[str]
    batch_size: NotRequired[int]
    timeout_ms: NotRequired[int]
    signature: list[QuerySignature]


class QueryFetch(TypedDict):
    rows: list[list[Any]]
    exhausted: bool


class Query:
    def __init__(self, peer: rpc.Peer, definition: QueryDefinition):
        self._peer = peer
        self.name = definition["name"]
        self.sql = definition.get("sql")
        self.prompt = definition.get("prompt")
        self.batch_size = definition.get("batch_size")
        self.timeout_ms = definition.get("timeout_ms")
        self.signature = definition["signature"]
        self.exhausted = False
        self._cache: list[list[Any]] = []
        self._cache_position = 0

    async def update(
        self,
        *,
        sql: str | None = None,
        prompt: str | None = None,
        batch_size: int | None = None,
        timeout_ms: int | None = None,
    ) -> None:
        params = _query_params(
            name=self.name,
            sql=sql,
            prompt=prompt,
            batch_size=batch_size,
            timeout_ms=timeout_ms,
        )
        definition: QueryDefinition = await self._peer.call("queries.update", params)
        self._replace(definition)

    async def reset(self) -> None:
        definition: QueryDefinition = await self._peer.call(
            "queries.reset",
            {"name": self.name},
        )
        self._replace(definition)

    async def delete(self) -> None:
        await self._peer.call("queries.delete", {"name": self.name})

    async def fetch(self) -> list[list[Any]]:
        if self.exhausted:
            return []

        response: QueryFetch = await self._peer.call(
            "queries.fetch",
            {"name": self.name},
        )
        self.exhausted = response["exhausted"]
        self._cache.extend(response["rows"])
        return response["rows"]

    def __aiter__(self) -> Query:
        return self

    async def __anext__(self) -> list[Any]:
        if self._cache_position < len(self._cache):
            row = self._cache[self._cache_position]
            self._cache_position += 1
            return row

        if self.exhausted:
            raise StopAsyncIteration

        await self.fetch()

        if self._cache_position < len(self._cache):
            row = self._cache[self._cache_position]
            self._cache_position += 1
            return row

        raise StopAsyncIteration

    def _replace(self, definition: QueryDefinition) -> None:
        self.name = definition["name"]
        self.sql = definition.get("sql")
        self.prompt = definition.get("prompt")
        self.batch_size = definition.get("batch_size")
        self.timeout_ms = definition.get("timeout_ms")
        self.signature = definition["signature"]
        self.exhausted = False
        self._cache = []
        self._cache_position = 0


class QueriesService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def create(
        self,
        name: str,
        *,
        sql: str | None = None,
        prompt: str | None = None,
        batch_size: int | None = None,
        timeout_ms: int | None = None,
    ) -> Query:
        params = _query_params(
            name=name,
            sql=sql,
            prompt=prompt,
            batch_size=batch_size,
            timeout_ms=timeout_ms,
        )
        definition: QueryDefinition = await self._peer.call("queries.create", params)
        return Query(self._peer, definition)

    async def get(self, name: str) -> Query:
        definition: QueryDefinition = await self._peer.call(
            "queries.get",
            {"name": name},
        )
        return Query(self._peer, definition)

    async def list(self) -> list[str]:
        response = await self._peer.call("queries.list")
        return response["queries"]


def _query_params(**values: Any) -> dict[str, Any]:
    return {key: value for key, value in values.items() if value is not None}
