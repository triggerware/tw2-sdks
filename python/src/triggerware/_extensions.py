from __future__ import annotations

from typing import Any, Literal, NotRequired, TypedDict

import tw2_jsonrpc as rpc


ExtensionRuntime = Literal["lua", "python", "node"]


class Extension(TypedDict):
    name: str
    tag: NotRequired[str]
    config: dict[str, Any] | None
    connectors: list[str]


class ExtensionDescription(TypedDict):
    description: str
    config_schema: dict[str, Any]
    runtime: NotRequired[ExtensionRuntime]
    dependencies: NotRequired[list[str]]
    connectors: list[str]


class ExtensionsService:
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
            "extensions.install",
            _extension_params(name=name, url=url, tag=tag, config=config or {}),
        )

    async def get(self, name: str, *, tag: str | None = None) -> Extension:
        response = await self._peer.call(
            "extensions.get",
            _extension_params(name=name, tag=tag),
        )
        extension: Extension = {
            "name": name,
            "config": response["config"],
            "connectors": response["connectors"],
        }
        if tag is not None:
            extension["tag"] = tag
        return extension

    async def list(self) -> list[str]:
        response = await self._peer.call("extensions.list")
        return response["extensions"]

    async def configure(
        self,
        name: str,
        *,
        tag: str | None = None,
        config: dict[str, Any],
    ) -> None:
        await self._peer.call(
            "extensions.configure",
            _extension_params(name=name, tag=tag, config=config),
        )

    async def catalog(self) -> list[str]:
        response = await self._peer.call("extensions.catalog")
        return response["extensions"]

    async def describe(
        self,
        *,
        name: str | None = None,
        url: str | None = None,
    ) -> ExtensionDescription:
        return await self._peer.call(
            "extensions.describe",
            _extension_params(name=name, url=url),
        )

    async def uninstall(self, name: str, *, tag: str | None = None) -> None:
        await self._peer.call(
            "extensions.uninstall",
            _extension_params(name=name, tag=tag),
        )


def _extension_params(**values: Any) -> dict[str, Any]:
    return {key: value for key, value in values.items() if value is not None}
