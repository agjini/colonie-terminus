#!/bin/bash
set -euo pipefail

INPUT_IMAGE="assets/tiles/magic/mainlevbuild.png"
OUTPUT_DIR="assets/tiles/magic"
BASENAME="mainlevbuild"

SCRIPTPATH="$(cd -- "$(dirname "$0")" >/dev/null 2>&1 && pwd -P)"
INPUT="${SCRIPTPATH}/${INPUT_IMAGE}"
OUTDIR="${SCRIPTPATH}/${OUTPUT_DIR}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "🗂 Temp dir: $TMPDIR"

# -----------------------------
# Detect tile size automatically
# -----------------------------
if command -v magick >/dev/null 2>&1; then
  SIZE=$(magick identify -format "%w %h" "$INPUT")
else
  SIZE=$(identify -format "%w %h" "$INPUT")
fi

WIDTH=${SIZE%% *}
HEIGHT=${SIZE##* }

gcd() {
  local a=$1 b=$2 t
  while (( b != 0 )); do
    t=$b; b=$((a % b)); a=$t
  done
  echo "$a"
}

TILE_SIZE=$(gcd "$WIDTH" "$HEIGHT")

echo "🔍 Image size: ${WIDTH}x${HEIGHT}"
echo "🧩 Detected tile size: ${TILE_SIZE}px"

# -----------------------------
# Split tiles
# -----------------------------
magick "$INPUT" -alpha on -background none \
  -crop "${TILE_SIZE}x${TILE_SIZE}" +repage \
  -define png:color-type=6 \
  "$TMPDIR/tile_%04d.png"

mapfile -t FILES < <(ls "$TMPDIR"/tile_*.png | sort)
LAYERS=${#FILES[@]}

echo "🧱 Tiles extracted: $LAYERS"

# -----------------------------
# Create KTX2 — UASTC
# -----------------------------
echo "⚡ Creating KTX2 (UASTC)..."

ktx create \
  --format R8G8B8A8_SRGB \
  --assign-tf srgb \
  --generate-mipmap \
  --encode uastc \
  --layers "$LAYERS" \
  "${FILES[@]}" \
  "$OUTDIR/${BASENAME}.ktx2"

# -----------------------------
# Create atlas PNG
# -----------------------------
echo "🧱 Creating atlas..."

ATLAS="$OUTDIR/${BASENAME}_atlas.png"
magick montage "${FILES[@]}" -tile x -geometry +0+0 "$ATLAS"

# -----------------------------
# JSON metadata
# -----------------------------
JSON="$OUTDIR/${BASENAME}_atlas.json"

cat > "$JSON" <<EOF
{
  "image": "$(basename "$ATLAS")",
  "tileSize": $TILE_SIZE,
  "columns": $(( WIDTH / TILE_SIZE )),
  "rows": $(( HEIGHT / TILE_SIZE )),
  "count": $LAYERS
}
EOF

echo "✅ Done!"
