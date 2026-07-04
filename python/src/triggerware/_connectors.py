from __future__ import annotations

import json
from typing import Any, Literal, NotRequired, TypedDict

import tw2_jsonrpc as rpc
import yaml


ConnectorRuntime = Literal["lua", "python", "node"]


class ConnectorColumn(TypedDict):
    name: str
    type: str


class ConnectorGenerator(TypedDict):
    name: str
    inputs: list[str]


class ConnectorMetadata(TypedDict):
    description: NotRequired[str]
    columns: list[ConnectorColumn]
    generators: list[ConnectorGenerator]
    config_schema: NotRequired[dict[str, Any]]


class ConnectorExtension(TypedDict):
    name: str
    tag: NotRequired[str]


class Connector(TypedDict):
    name: str
    meta: ConnectorMetadata
    code: str
    runtime: NotRequired[ConnectorRuntime]
    extension: NotRequired[ConnectorExtension]
    configured: bool


class ConnectorsService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def create(
        self,
        name: str,
        *,
        prompt: str | None = None,
        code: str | None = None,
        meta: ConnectorMetadata | str | None = None,
        runtime: ConnectorRuntime | None = None,
    ) -> Connector:
        connector = await self._peer.call(
            "connectors.create",
            self._definition_params(
                name=name,
                prompt=prompt,
                code=code,
                meta=meta,
                runtime=runtime,
            ),
        )
        connector["name"] = name
        return connector

    async def get(self, name: str) -> Connector:
        connector = await self._peer.call("connectors.get", {"name": name})
        connector["name"] = name
        return connector

    async def update(
        self,
        name: str,
        *,
        prompt: str | None = None,
        code: str | None = None,
        meta: ConnectorMetadata | str | None = None,
        runtime: ConnectorRuntime | None = None,
    ) -> Connector:
        connector = await self._peer.call(
            "connectors.update",
            self._definition_params(
                name=name,
                prompt=prompt,
                code=code,
                meta=meta,
                runtime=runtime,
            ),
        )
        connector["name"] = name
        return connector

    async def list(self) -> list[str]:
        response = await self._peer.call("connectors.list")
        return response["connectors"]

    async def delete(self, name: str) -> None:
        await self._peer.call("connectors.delete", {"name": name})

    async def sync(
        self,
        *,
        connectors: list[str] | None = None,
        extensions: list[str] | None = None,
    ) -> None:
        params = {
            "connectors": connectors,
            "extensions": extensions,
        }
        await self._peer.call(
            "connectors.sync",
            {key: value for key, value in params.items() if value is not None},
        )

    async def configure(self, connector: str | Connector, **config: Any) -> None:
        if isinstance(connector, str):
            name = connector
            attached_connector = None
        else:
            name = connector["name"]
            attached_connector = connector

        await self._peer.call(
            "connectors.configure",
            {
                "name": name,
                "config": config,
            },
        )

        if attached_connector is not None:
            attached_connector["configured"] = True

    def _definition_params(
        self,
        *,
        name: str,
        prompt: str | None,
        code: str | None,
        meta: ConnectorMetadata | str | None,
        runtime: ConnectorRuntime | None,
    ) -> dict[str, Any]:
        params: dict[str, Any] = {"name": name}

        if prompt is not None:
            params["prompt"] = prompt
        else:
            if code is None or meta is None:
                raise ValueError("either prompt or both code and meta must be supplied")
            params["code"] = code
            params["meta"] = connector_metadata_from_string(meta) if isinstance(meta, str) else meta

        if runtime is not None:
            params["runtime"] = runtime

        return params


def connector_metadata_from_string(contents: str) -> ConnectorMetadata:
    try:
        parsed = yaml.safe_load(contents)
    except yaml.YAMLError:
        parsed = json.loads(contents)

    if not isinstance(parsed, dict):
        raise ValueError("connector metadata must be an object")
    return parsed  # type: ignore[return-value]
