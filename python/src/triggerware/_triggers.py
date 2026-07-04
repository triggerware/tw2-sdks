from __future__ import annotations

from inspect import isawaitable
from typing import Any, Awaitable, Callable, NotRequired, TypedDict

import tw2_jsonrpc as rpc


class TriggerSignature(TypedDict):
    name: str
    type: str


class TriggerDefinition(TypedDict):
    name: str
    prompt: NotRequired[str]
    sql: NotRequired[str]
    schedule: NotRequired[str]
    delivery: NotRequired[list[str]]
    batch_size: NotRequired[int]
    timeout_ms: NotRequired[int]
    report_noops: bool
    enabled: bool
    signature: list[TriggerSignature]


class TriggerDelta(TypedDict):
    added: list[list[Any]]
    deleted: list[list[Any]]


TriggerCallback = Callable[
    [list[list[Any]], list[list[Any]]],
    None | Awaitable[None],
]


class Trigger:
    def __init__(self, peer: rpc.Peer, definition: TriggerDefinition):
        self._peer = peer
        self._resume_token: str | None = None
        self._replace(definition)

    @property
    def is_watched(self) -> bool:
        return self._resume_token is not None

    async def update(
        self,
        *,
        sql: str | None = None,
        prompt: str | None = None,
        schedule: str | None = None,
        delivery: list[str] | None = None,
        batch_size: int | None = None,
        timeout_ms: int | None = None,
        report_noops: bool | None = None,
        enabled: bool | None = None,
    ) -> None:
        params = _trigger_params(
            name=self.name,
            sql=sql,
            prompt=prompt,
            schedule=schedule,
            delivery=delivery,
            batch_size=batch_size,
            timeout_ms=timeout_ms,
            report_noops=report_noops,
            enabled=enabled,
        )
        definition: TriggerDefinition = await self._peer.call("triggers.update", params)
        self._replace(definition)

    async def delete(self) -> None:
        await self._peer.call("triggers.delete", {"name": self.name})

    async def enable(self) -> None:
        await self._peer.call("triggers.enable", {"name": self.name})
        await self._refresh()

    async def disable(self) -> None:
        await self._peer.call("triggers.disable", {"name": self.name})
        await self._refresh()

    async def run(
        self,
        *,
        batch_size: int | None = None,
        timeout_ms: int | None = None,
    ) -> TriggerDelta:
        return await self._peer.call(
            "triggers.run",
            _trigger_params(
                name=self.name,
                batch_size=batch_size,
                timeout_ms=timeout_ms,
            ),
        )

    async def poll(self) -> list[TriggerDelta]:
        response = await self._peer.call("triggers.poll", {"name": self.name})
        return response["deltas"]

    async def watch(
        self,
        callback: TriggerCallback,
        *,
        resume_window_ms: int | None = None,
    ) -> str:
        if self._resume_token is not None:
            raise RuntimeError("trigger is already watching")

        self._register_watcher(callback)
        response = await self._peer.call(
            "triggers.watch",
            _trigger_params(
                name=self.name,
                resume_window_ms=resume_window_ms,
            ),
        )
        self._resume_token = response["resume_token"]
        return self._resume_token

    async def resume(
        self,
        resume_token: str,
        callback: TriggerCallback,
        *,
        resume_window_ms: int | None = None,
    ) -> str:
        if self._resume_token is not None:
            raise RuntimeError("trigger is already watching")

        self._register_watcher(callback)
        response = await self._peer.call(
            "triggers.resume",
            _trigger_params(
                name=self.name,
                resume_token=resume_token,
                resume_window_ms=resume_window_ms,
            ),
        )
        self._resume_token = response["resume_token"]
        return self._resume_token

    async def unwatch(self) -> None:
        if self._resume_token is None:
            return

        await self._peer.call(
            "triggers.unwatch",
            {"resume_token": self._resume_token},
        )
        self._resume_token = None

    async def _refresh(self) -> None:
        definition: TriggerDefinition = await self._peer.call(
            "triggers.get",
            {"name": self.name},
        )
        self._replace(definition)

    def _replace(self, definition: TriggerDefinition) -> None:
        self.name = definition["name"]
        self.sql = definition.get("sql")
        self.prompt = definition.get("prompt")
        self.schedule = definition.get("schedule")
        self.delivery = definition.get("delivery", [])
        self.batch_size = definition.get("batch_size")
        self.timeout_ms = definition.get("timeout_ms")
        self.report_noops = definition["report_noops"]
        self.enabled = definition["enabled"]
        self.signature = definition["signature"]

    def _register_watcher(self, callback: TriggerCallback) -> None:
        async def handler(params: dict[str, Any]) -> None:
            for delta in params["deltas"]:
                result = callback(delta["added"], delta["deleted"])
                if isawaitable(result):
                    await result

        self._peer.add_method(self.name, handler)


class TriggersService:
    def __init__(self, peer: rpc.Peer):
        self._peer = peer

    async def create(
        self,
        name: str,
        *,
        sql: str | None = None,
        prompt: str | None = None,
        schedule: str | None = None,
        delivery: list[str] | None = None,
        batch_size: int | None = None,
        timeout_ms: int | None = None,
        report_noops: bool | None = None,
        enabled: bool | None = None,
    ) -> Trigger:
        definition: TriggerDefinition = await self._peer.call(
            "triggers.create",
            _trigger_params(
                name=name,
                sql=sql,
                prompt=prompt,
                schedule=schedule,
                delivery=delivery,
                batch_size=batch_size,
                timeout_ms=timeout_ms,
                report_noops=report_noops,
                enabled=enabled,
            ),
        )
        return Trigger(self._peer, definition)

    async def get(self, name: str) -> Trigger:
        definition: TriggerDefinition = await self._peer.call(
            "triggers.get",
            {"name": name},
        )
        return Trigger(self._peer, definition)

    async def list(self) -> list[str]:
        response = await self._peer.call("triggers.list")
        return response["triggers"]


def _trigger_params(**values: Any) -> dict[str, Any]:
    return {key: value for key, value in values.items() if value is not None}
