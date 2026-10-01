"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class ProjectAuthorityHandlers(Protocol):
    async def authority_open(self, project_id: str) -> ProjectOpen: ...
    async def authority_acquire_writer(self, request: AcquireWriter) -> WriterLease: ...

def register(transport, schema, handlers: "ProjectAuthorityHandlers") -> None:
    bind = {
        "authority.open": handlers.authority_open,
        "authority.acquire_writer": handlers.authority_acquire_writer,
    }
    for m in schema.services["ProjectAuthority"].methods:
        transport.register_method(m, bind[m.name])
