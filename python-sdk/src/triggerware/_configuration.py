from __future__ import annotations

from typing import NotRequired, TypedDict

import pysonrpc as rpc


class Configuration(TypedDict):
    default_timeout_ms: NotRequired[int]
    default_batch_size: NotRequired[int]
    default_trigger_batch_size: NotRequired[int]
    llm_provider: NotRequired[str]
    llm_model: NotRequired[str]
    max_log_file_size: NotRequired[str]


class ConfigurationService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def get(self) -> Configuration:
        return await self._peer.call("configuration.get")

    async def set(
        self,
        *,
        default_timeout_ms: int | None = None,
        default_batch_size: int | None = None,
        default_trigger_batch_size: int | None = None,
        llm_provider: str | None = None,
        llm_model: str | None = None,
        llm_api_key: str | None = None,
        max_log_file_size: str | None = None,
    ) -> Configuration:
        params = {
            "default_timeout_ms": default_timeout_ms,
            "default_batch_size": default_batch_size,
            "default_trigger_batch_size": default_trigger_batch_size,
            "llm_provider": llm_provider,
            "llm_model": llm_model,
            "llm_api_key": llm_api_key,
            "max_log_file_size": max_log_file_size,
        }
        return await self._peer.call(
            "configuration.set",
            {key: value for key, value in params.items() if value is not None},
        )

    async def refresh(self) -> Configuration:
        return await self._peer.call("configuration.refresh")
