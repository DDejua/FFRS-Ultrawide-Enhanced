#!/usr/bin/env python3
"""Prepare a manual-only FFRS v1.1.1 ZIP from the SHA-pinned public v1.1.0 ZIP.

Never download an arbitrary mutable loader. Never change HUD preset/TOML or
license file bytes. Only the game's ASI and release-facing text differ.
"""
from __future__ import annotations
import hashlib
import io
import struct
import sys
import zipfile
from pathlib import Path

ORIGINAL_ZIP_SHA256 = "a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0"
ORIGINAL_ASI_SHA256 = "7c9b091522ad8e4606895218f34690551254e0b5c13426abbfefd53a057015d1"
LOADER_SHA256 = "900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937"
# Exact machine instructions of the author-tested Cloud/Terra/Bahamut/WoL candidate.
TESTED_TEXT_SHA256 = "3baa442275fae74788c4932fdeaa97fc17e71be78de6529b3eea02e60646a1cf"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def pe_text_sha(binary: bytes) -> str:
    pe = struct.unpack_from("<I", binary, 0x3c)[0]
    if binary[pe:pe + 4] != b"PE\x00\x00":
        raise RuntimeError("Invalid Windows PE module")
    count = struct.unpack_from("<H", binary, pe + 6)[0]
    optional_size = struct.unpack_from("<H", binary, pe + 20)[0]
    section_start = pe + 24 + optional_size
    for index in range(count):
        start = section_start + 40 * index
        name = binary[start:start + 8].rstrip(b"\0")
        if name == b".text":
            size = struct.unpack_from("<I", binary, start + 16)[0]
            offset = struct.unpack_from("<I", binary, start + 20)[0]
            return sha(binary[offset:offset + size])
    raise RuntimeError("Missing .text section")


def main() -> None:
    if len(sys.argv) != 6:
        raise SystemExit("usage: package-v111.py APPROVED_V110_ZIP COMPILED_DLL OUTPUT_ZIP SOURCE_SHA BUILD_RUN_URL")
    old_zip, dll_file, output, source_commit, build_url = map(str, sys.argv[1:])
    old_data = Path(old_zip).read_bytes()
    if sha(old_data) != ORIGINAL_ZIP_SHA256:
        raise RuntimeError("v1.1.0 release ZIP SHA256 mismatch; no packaging allowed")
    with zipfile.ZipFile(io.BytesIO(old_data)) as z:
        files = {name: z.read(name) for name in z.namelist() if not name.endswith("/")}
    if sha(files["winmm.dll"]) != LOADER_SHA256:
        raise RuntimeError("Vetted original winmm.dll differs")
    if sha(files["scripts/final_fantasy_resonance_fix.asi"]) != ORIGINAL_ASI_SHA256:
        raise RuntimeError("v1.1.0 baseline ASI differs")
    new_asi = Path(dll_file).read_bytes()
    text_hash = pe_text_sha(new_asi)
    if text_hash != TESTED_TEXT_SHA256:
        raise RuntimeError("New compiled game code differs from author-tested candidate: " + text_hash)
    preset_files = [s for s in files if s.startswith("HUD_PRESETS/") and s.endswith(".toml")]
    if len(preset_files) != 3:
        raise RuntimeError("Not all three old HUD presets are present")
    legal_files = [s for s in files if s.startswith("LICENSES/")]
    if len(legal_files) < 4:
        raise RuntimeError("Third-party legal notices missing")

    release_notes = Path("RELEASE_NOTES_v1.1.1.md").read_text(encoding="utf-8")
    preserved = {key: sha(files[key]) for key in files
                 if key != "scripts/final_fantasy_resonance_fix.asi"}
    files["scripts/final_fantasy_resonance_fix.asi"] = new_asi
    before = files["README.txt"].decode("utf-8-sig")
    intro = """FINAL FANTASY RESONANCE — ULTRAWIDE FIX + DOF v1.1.1
===================================================
VISION FMV TRANSPARENCY PATCH — MANUAL INSTALL ONLY
- Corrects unintended black background behind Cloud/Terra Vision intros.
- Retains black outside 16:9 bars and all v1.1.0 DOF, HUD, menu and crash fixes.
- Visually tested at 3440x1440 in HYBRID, ULTRAWIDE and CLASSIC.
- 32:9/higher are supported by dynamic calculations, NOT hardware-tested.
- From v1.1.0: BACK UP then replace ONLY scripts/final_fantasy_resonance_fix.asi.
- Do NOT overwrite your existing .toml or winmm.dll for a routine update.
- Keep [movies] enable = true, then restart the game.
- New installations: refer to original manual installation guide below.
- No scripts or automatic installers are present in this ZIP.

=========== PREVIOUS v1.1.0 FULL INSTRUCTIONS ===========
"""
    files["README.txt"] = (intro + before).encode("utf-8")
    files["CHANGELOG_v1.1.1.txt"] = release_notes.encode("utf-8")
    manifest = [
        "FFRS Ultrawide Fix + DOF v1.1.1",
        "Release-built source SHA: " + source_commit,
        "GitHub Actions build URL: " + build_url,
        "Original approved v1.1.0 player ZIP SHA256: " + ORIGINAL_ZIP_SHA256,
        "New ASI SHA256: " + sha(new_asi),
        "New ASI .text SHA256 (matches user-tested executable code): " + text_hash,
        "Original ThirteenAG winmm.dll SHA256: " + LOADER_SHA256,
        "Previous v1.1.0 ASI SHA256: " + ORIGINAL_ASI_SHA256,
        "The exact original loader, TOML presets, configuration and licenses are preserved.",
        "32:9 and higher not verified on physical hardware.",
        "MANUAL INSTALLATION ONLY.",
        "",
        "File checksums (excludes this manifest):",
    ]
    for name, data in sorted(files.items()):
        if name != "BUILD/RELEASE_INFO_AND_HASHES.txt":
            manifest.append(f"{sha(data)}  {name}")
    files["BUILD/RELEASE_INFO_AND_HASHES.txt"] = ("\n".join(manifest) + "\n").encode()
    allowed_changes = {"scripts/final_fantasy_resonance_fix.asi",
                       "README.txt", "BUILD/RELEASE_INFO_AND_HASHES.txt"}
    if any(name.lower().endswith((".cmd", ".bat", ".ps1", ".exe", ".msi", ".sh")) for name in files):
        raise RuntimeError("Forbidden executable installer or script in player archive")
    for name, old_hash in preserved.items():
        if name not in allowed_changes and sha(files[name]) != old_hash:
            raise RuntimeError("A v1.1.0 file was changed unexpectedly: " + name)

    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=6) as z:
        for name, content in files.items():
            z.writestr(name, content)
    with zipfile.ZipFile(output) as z:
        if set(z.namelist()) != set(files):
            raise RuntimeError("Missing/duplicate archive entries")
        for name, content in files.items():
            if z.read(name) != content:
                raise RuntimeError("ZIP content verification failed: " + name)
    print("Release package:", output)
    print("SHA256:", sha(Path(output).read_bytes()))
    print("ASI SHA256:", sha(new_asi))
    print("Preserved loader SHA256:", sha(files["winmm.dll"]))
    print("Author-tested .text SHA256:", text_hash)


if __name__ == "__main__":
    main()
