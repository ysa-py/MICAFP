"""Tests for self_heal.py file discovery."""

import os
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from self_heal import iter_python_files


def test_iter_python_files_includes_nested_project_scripts():
    discovered = {Path(path).as_posix() for path in iter_python_files()}

    assert "torshield_ai_gateway/gateway.py" in discovered
    assert "scripts/run_full_audit.py" in discovered
