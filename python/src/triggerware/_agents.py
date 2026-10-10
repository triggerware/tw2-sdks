from __future__ import annotations

from typing import Any

import tw2_jsonrpc as rpc


class AgentsService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def run(self, name: str, data: dict[str, Any]) -> None:
        await self._peer.call("agents.run", {"name": name, "data": data})
