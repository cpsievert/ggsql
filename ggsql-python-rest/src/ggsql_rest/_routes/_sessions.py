"""Session management routes."""

from fastapi import APIRouter, Depends, HTTPException

from .._models import SessionResponse, TablesResponse
from .._sessions import Session, SessionManager

router = APIRouter(prefix="/sessions", tags=["sessions"])

# Dependency placeholder - will be overridden by app factory
_session_manager: SessionManager | None = None


def get_session_manager() -> SessionManager:
    """Get the session manager instance."""
    if _session_manager is None:
        raise RuntimeError("SessionManager not initialized")
    return _session_manager


def get_session(
    session_id: str,
    session_mgr: SessionManager = Depends(get_session_manager),
) -> Session:
    """Get a session by ID or raise 404."""
    session = session_mgr.get(session_id)
    if session is None:
        raise HTTPException(404, f"Session '{session_id}' not found")
    return session


@router.post("", response_model=SessionResponse)
def create_session(
    session_mgr: SessionManager = Depends(get_session_manager),
) -> SessionResponse:
    """Create a new session."""
    session = session_mgr.create()
    return SessionResponse(session_id=session.id)


@router.delete("/{session_id}")
def delete_session(
    session_id: str,
    session_mgr: SessionManager = Depends(get_session_manager),
) -> dict:
    """Delete a session."""
    if not session_mgr.delete(session_id):
        raise HTTPException(404, f"Session '{session_id}' not found")
    return {"status": "deleted"}


@router.get("/{session_id}/tables", response_model=TablesResponse)
def list_tables(session: Session = Depends(get_session)) -> TablesResponse:
    """List tables available in a session."""
    return TablesResponse(tables=session.tables)
