from __future__ import annotations

import tw2_jsonrpc as rpc


class PromptsService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def get(self, name: str) -> str:
        response = await self._peer.call("prompts.get", {"name": name})
        return response["prompt"]

    async def set(self, name: str, prompt: str) -> None:
        await self._peer.call(
            "prompts.set",
            {
                "name": name,
                "prompt": prompt,
            },
        )

    async def reset(self, name: str | None = None) -> None:
        params = {} if name is None else {"name": name}
        await self._peer.call("prompts.reset", params)
