import socket
import ssl
import sys
import types
from unittest.mock import Mock, patch

import pytest

import ech_fingerprint_evasion as efe


@pytest.mark.parametrize(
    ("exc", "status"),
    [
        (socket.timeout("timed out"), "timeout"),
        (TimeoutError("timed out"), "timeout"),
        (ConnectionRefusedError("refused"), "connection_refused"),
        (OSError("network unreachable"), "unreachable"),
        (ssl.SSLError("tls handshake failed"), "ssl_error"),
    ],
)
def test_check_ech_records_expected_probe_failures_without_global_telemetry(exc, status):
    recorder = Mock()
    fake_logger = types.ModuleType("monitoring.structured_logger")
    fake_logger.record_silent_failure = recorder

    with patch.dict(sys.modules, {"monitoring.structured_logger": fake_logger}):
        with patch("ech_fingerprint_evasion.socket.create_connection", side_effect=exc):
            result = efe._check_ech("198.51.100.7", 443)

    assert result["tls_reachable"] is False
    assert result["tls_probe_status"] == status
    assert result["tls_error_type"] == type(exc).__name__
    recorder.assert_not_called()


def test_check_ech_keeps_unexpected_probe_failures_visible_and_telemetered():
    recorder = Mock()
    fake_logger = types.ModuleType("monitoring.structured_logger")
    fake_logger.record_silent_failure = recorder
    exc = RuntimeError("parser bug")

    with patch.dict(sys.modules, {"monitoring.structured_logger": fake_logger}):
        with patch("ech_fingerprint_evasion.socket.create_connection", side_effect=exc):
            result = efe._check_ech("198.51.100.7", 443)

    assert result["tls_probe_status"] == "unexpected_error"
    assert result["tls_error_type"] == "RuntimeError"
    recorder.assert_called_once_with("ech_fingerprint_evasion:56", exc)
