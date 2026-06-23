from __future__ import annotations

from typing import Literal

import pysonrpc as rpc


Runtime = Literal["python", "node"]


class RuntimesService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def list(self) -> list[str]:
        response = await self._peer.call("runtimes.list")
        return response["runtimes"]

    async def install(self, name: Runtime) -> None:
        await self._peer.call("runtimes.install", {"name": name})

    async def uninstall(self, name: Runtime) -> None:
        await self._peer.call("runtimes.uninstall", {"name": name})
