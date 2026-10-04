import datetime
import json
import os
from pathlib import Path
import sys
import tomllib
import urllib.parse
import urllib.request


CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
COOLDOWN = datetime.timedelta(days=3)


def packages(lockfile):
    return {
        (package["name"], package["version"], package["source"])
        for package in tomllib.loads(lockfile.decode())["package"]
        if "source" in package
    }


def publication_time(name, version):
    url = "https://crates.io/api/v1/crates/{}/{}".format(
        urllib.parse.quote(name, safe=""), urllib.parse.quote(version, safe="")
    )
    request = urllib.request.Request(
        url,
        headers={
            "User-Agent": "compare-changes-cargo-update (https://github.com/anttiharju/compare-changes)",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(request, timeout=30) as response:
        metadata = json.load(response)["version"]
    if metadata["crate"] != name or metadata["num"] != version:
        raise ValueError(f"Publication metadata does not match {name} {version}.")
    published = datetime.datetime.fromisoformat(metadata["created_at"])
    if published.tzinfo is None:
        raise ValueError(f"Publication time has no time zone for {name} {version}.")
    return published


def eligible(before, after, now):
    if before == after:
        print("Cargo.lock has no changes.")
        return False
    cutoff = now - COOLDOWN
    for name, version, source in sorted(packages(after) - packages(before)):
        if source != CRATES_IO:
            raise ValueError(f"No publication metadata source for {name} {version}: {source}")
        if publication_time(name, version) > cutoff:
            print(f"The update waits until {name} {version} is at least 72 hours old.")
            return False
    print("Every new crate version is at least 72 hours old.")
    return True


def main():
    before = Path(sys.argv[1]).read_bytes()
    after = Path("Cargo.lock").read_bytes()
    changed = eligible(before, after, datetime.datetime.now(datetime.timezone.utc))
    with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
        output.write(f"changed={str(changed).lower()}\n")


if __name__ == "__main__":
    main()
