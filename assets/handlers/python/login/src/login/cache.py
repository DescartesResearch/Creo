import secrets
from dataclasses import dataclass, field
from functools import lru_cache

from login.models import SessionData, SessionResponse


@dataclass
class _SessionRepository:
    _cache: dict[str, SessionData] = field(default_factory=dict)

    def set_new_session(self, user_id: str) -> SessionResponse:
        session_id = secrets.token_urlsafe(24)
        session_data = SessionData(user_id=user_id, session_id=session_id)
        self._cache[session_id] = session_data

        return SessionResponse(token=session_id, exp=session_data.exp)


@lru_cache
def get_session_repository() -> _SessionRepository:
    return _SessionRepository()
