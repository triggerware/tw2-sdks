from __future__ import annotations

from typing import Any, Literal, NotRequired, TypedDict

import tw2_jsonrpc as rpc


ConnectorRuntime = Literal["lua", "python", "node"]


class Connector(TypedDict):
    name: str
    tag: NotRequired[str]
    config: dict[str, Any] | None
    tables: list[str]
    tools: list[str]
    agents: list[str]


class ConnectorDescription(TypedDict):
    description: str
    config_schema: dict[str, Any]
    runtime: NotRequired[ConnectorRuntime]
    dependencies: NotRequired[list[str]]
    tables: list[str]
    tools: list[str]
    agents: list[str]


class ConnectorsService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def install(
        self,
        *,
        name: str | None = None,
        url: str | None = None,
        tag: str | None = None,
        config: dict[str, Any] | None = None,
    ) -> None:
        await self._peer.call(
            "connectors.install",
            _connector_params(name=name, url=url, tag=tag, config=config or {}),
        )

    async def get(self, name: str, *, tag: str | None = None) -> Connector:
        response = await self._peer.call(
            "connectors.get",
            _connector_params(name=name, tag=tag),
        )
        connector: Connector = {
            "name": name,
            "config": response["config"],
            "tables": response["tables"],
            "tools": response["tools"],
            "agents": response["agents"],
        }
        if tag is not None:
            connector["tag"] = tag
        return connector

    async def list(self) -> list[str]:
        response = await self._peer.call("connectors.list")
        return response["connectors"]

    async def configure(
        self,
        name: str,
        *,
        tag: str | None = None,
        config: dict[str, Any],
    ) -> None:
        await self._peer.call(
            "connectors.configure",
            _connector_params(name=name, tag=tag, config=config),
        )

    async def catalog(self) -> list[str]:
        response = await self._peer.call("connectors.catalog")
        return response["connectors"]

    async def describe(
        self,
        *,
        name: str | None = None,
        url: str | None = None,
    ) -> ConnectorDescription:
        return await self._peer.call(
            "connectors.describe",
            _connector_params(name=name, url=url),
        )

    async def uninstall(self, name: str, *, tag: str | None = None) -> None:
        await self._peer.call(
            "connectors.uninstall",
            _connector_params(name=name, tag=tag),
        )


def _connector_params(**values: Any) -> dict[str, Any]:
    return {key: value for key, value in values.items() if value is not None}
