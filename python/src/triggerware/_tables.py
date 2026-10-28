from __future__ import annotations

import json
from typing import Any, Literal, NotRequired, TypedDict

import tw2_jsonrpc as rpc
import yaml


TableRuntime = Literal["lua", "python", "node"]


class TableColumn(TypedDict):
    name: str
    type: str


class TableGenerator(TypedDict):
    name: str
    inputs: list[str]


class TableMetadata(TypedDict):
    description: NotRequired[str]
    columns: list[TableColumn]
    generators: list[TableGenerator]
    config_schema: NotRequired[dict[str, Any]]


class TableConnector(TypedDict):
    name: str
    tag: NotRequired[str]


class Table(TypedDict):
    name: str
    meta: TableMetadata
    code: str
    runtime: NotRequired[TableRuntime]
    connector: NotRequired[TableConnector]
    configured: bool


class TablesService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def create(
        self,
        name: str,
        *,
        prompt: str | None = None,
        code: str | None = None,
        meta: TableMetadata | str | None = None,
        runtime: TableRuntime | None = None,
    ) -> Table:
        table = await self._peer.call(
            "tables.create",
            self._definition_params(
                name=name,
                prompt=prompt,
                code=code,
                meta=meta,
                runtime=runtime,
            ),
        )
        table["name"] = name
        return table

    async def get(self, name: str) -> Table:
        table = await self._peer.call("tables.get", {"name": name})
        table["name"] = name
        return table

    async def update(
        self,
        name: str,
        *,
        prompt: str | None = None,
        code: str | None = None,
        meta: TableMetadata | str | None = None,
        runtime: TableRuntime | None = None,
    ) -> Table:
        table = await self._peer.call(
            "tables.update",
            self._definition_params(
                name=name,
                prompt=prompt,
                code=code,
                meta=meta,
                runtime=runtime,
            ),
        )
        table["name"] = name
        return table

    async def list(self) -> list[str]:
        response = await self._peer.call("tables.list")
        return response["tables"]

    async def delete(self, name: str) -> None:
        await self._peer.call("tables.delete", {"name": name})

    async def sync(
        self,
        *,
        tables: list[str] | None = None,
        connectors: list[str] | None = None,
    ) -> None:
        params = {
            "tables": tables,
            "connectors": connectors,
        }
        await self._peer.call(
            "tables.sync",
            {key: value for key, value in params.items() if value is not None},
        )

    async def configure(self, table: str | Table, **config: Any) -> None:
        if isinstance(table, str):
            name = table
            attached_table = None
        else:
            name = table["name"]
            attached_table = table

        await self._peer.call(
            "tables.configure",
            {
                "name": name,
                "config": config,
            },
        )

        if attached_table is not None:
            attached_table["configured"] = True

    def _definition_params(
        self,
        *,
        name: str,
        prompt: str | None,
        code: str | None,
        meta: TableMetadata | str | None,
        runtime: TableRuntime | None,
    ) -> dict[str, Any]:
        params: dict[str, Any] = {"name": name}

        if prompt is not None:
            params["prompt"] = prompt
        else:
            if code is None or meta is None:
                raise ValueError("either prompt or both code and meta must be supplied")
            params["code"] = code
            params["meta"] = table_metadata_from_string(meta) if isinstance(meta, str) else meta

        if runtime is not None:
            params["runtime"] = runtime

        return params


def table_metadata_from_string(contents: str) -> TableMetadata:
    try:
        parsed = yaml.safe_load(contents)
    except yaml.YAMLError:
        parsed = json.loads(contents)

    if not isinstance(parsed, dict):
        raise ValueError("table metadata must be an object")
    return parsed  # type: ignore[return-value]
