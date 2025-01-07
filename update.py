"""Automates the process of updating seaside for those who choose to
compile the source code themselves.

This script preserves your current copy of 'Seaside.toml', so no
settings should be lost in this process; however, you may need to
manually upgrade your settings to accomodate new versions. Another
script may be made to do that in the future.
"""

from __future__ import annotations
from hashlib import sha256
from os import remove as remove_file
from pathlib import Path
from shutil import copy2
from subprocess import run
from sys import platform
from tempfile import NamedTemporaryFile


CONFIG_NAME: str = "Seaside.toml"


def get_build_command() -> list[str]:
    """Forms a cargo build command based on the platform."""

    command: list[str] = ["cargo", "build", "--release"]
    if platform.startswith("linux") or platform == "darwin":
        command.insert(0, "-Znext-lockfile-bump")
    return command


class BackupFileManager:
    """A context manager which automatically backs up
    and restores a file.

    Restoration only occurs if the file has changed.
    """

    __slots__: tuple[str, ...] = (
        "file_path",
        "backup_path",
        "original_hash",
    )

    def __init__(self, file_name: str) -> None:
        self.file_path: Path = Path(file_name)

    def __enter__(self) -> BackupFileManager:
        with open(self.file_path, 'rb') as config:
            self.original_hash = sha256(config.read())
        with NamedTemporaryFile(delete=False) as backup:
            self.backup_path: str = backup.name
            copy2(self.file_path, self.backup_path)
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> bool:
        new_hash = None
        if self.file_path.exists():
            with open(self.file_path, 'rb') as config:
                new_hash = sha256(config.read())
        if new_hash is None or new_hash != self.original_hash:
            copy2(self.backup_path, self.file_path)
        remove_file(self.backup_path)
        return False  # propagate exceptions


def main() -> None:
    with BackupFileManager(CONFIG_NAME) as _:
        run(["git", "pull"], check=True)
    run(get_build_command(), check=True)


if __name__ == "__main__":
    main()
