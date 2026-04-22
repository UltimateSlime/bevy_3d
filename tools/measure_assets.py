"""Measure dimensions and pivot offsets of all medieval gltf assets.

Usage:
    python tools/measure_assets.py

Reads all .gltf files under assets/medieval/ and writes results to
tools/asset_measurements.md for easy reference.

Per asset we compute:
    - size (x, y, z): actual bounding box dimensions
    - min / max: bounding box extents in local coordinates
    - pivot_offset: offset from the mesh's geometric center to the pivot (origin)
                    If (0, 0, 0) the pivot IS the center.
"""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ASSETS_DIR = ROOT / "assets" / "medieval"
OUTPUT_FILE = Path(__file__).resolve().parent / "asset_measurements.md"

CATEGORIES = [
    "Wall", "Roof", "Door", "DoorFrame", "Window", "WindowShutters",
    "Floor", "Corner", "Stair", "Stairs",
    "Overhang", "Balcony", "Prop", "HoleCover",
]


def gather_bounds(gltf_data):
    """Return (min, max) over all mesh POSITION accessors, or None if absent."""
    all_min = [float("inf")] * 3
    all_max = [float("-inf")] * 3
    found = False

    for mesh in gltf_data.get("meshes", []):
        for prim in mesh.get("primitives", []):
            pos_idx = prim.get("attributes", {}).get("POSITION")
            if pos_idx is None:
                continue
            acc = gltf_data["accessors"][pos_idx]
            if "min" not in acc or "max" not in acc:
                continue
            for i in range(3):
                all_min[i] = min(all_min[i], acc["min"][i])
                all_max[i] = max(all_max[i], acc["max"][i])
            found = True

    if not found:
        return None
    return all_min, all_max


def measure_asset(gltf_path):
    with open(gltf_path) as f:
        data = json.load(f)

    bounds = gather_bounds(data)
    if bounds is None:
        return None
    min_v, max_v = bounds

    size = [max_v[i] - min_v[i] for i in range(3)]
    # center of bounding box relative to the pivot (origin)
    center_offset = [(max_v[i] + min_v[i]) / 2 for i in range(3)]

    return {
        "name": gltf_path.stem,
        "size": size,
        "min": min_v,
        "max": max_v,
        "pivot_offset": center_offset,
    }


def category_of(name):
    for cat in CATEGORIES:
        if name.startswith(cat):
            return cat
    return "Other"


def main():
    results = []
    for gltf_file in sorted(ASSETS_DIR.glob("*.gltf")):
        r = measure_asset(gltf_file)
        if r is not None:
            results.append(r)

    # group by category
    grouped = {}
    for r in results:
        grouped.setdefault(category_of(r["name"]), []).append(r)

    # write markdown
    lines = []
    lines.append("# Medieval Asset Measurements\n")
    lines.append(f"Total assets measured: **{len(results)}**\n")
    lines.append("All values in meters. `pivot_offset` is the offset from the pivot (origin) to the geometric center of the mesh. If it is (0, 0, 0), the pivot IS the center.\n")

    order = CATEGORIES + ["Other"]
    for cat in order:
        items = grouped.get(cat)
        if not items:
            continue
        lines.append(f"\n## {cat} ({len(items)})\n")
        lines.append("| Name | Size (X, Y, Z) | Min | Max | Pivot offset |")
        lines.append("|---|---|---|---|---|")
        for r in items:
            sx, sy, sz = r["size"]
            mnx, mny, mnz = r["min"]
            mxx, mxy, mxz = r["max"]
            px, py, pz = r["pivot_offset"]
            lines.append(
                f"| `{r['name']}` "
                f"| ({sx:.2f}, {sy:.2f}, {sz:.2f}) "
                f"| ({mnx:.2f}, {mny:.2f}, {mnz:.2f}) "
                f"| ({mxx:.2f}, {mxy:.2f}, {mxz:.2f}) "
                f"| ({px:.2f}, {py:.2f}, {pz:.2f}) |"
            )

    OUTPUT_FILE.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote {len(results)} assets to {OUTPUT_FILE}")


if __name__ == "__main__":
    main()