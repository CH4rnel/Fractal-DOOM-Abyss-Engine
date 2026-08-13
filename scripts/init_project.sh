#!/usr/bin/env bash

set -euo pipefail

echo "======================================="
echo " FRACTAL DOOM :: ABYSS ENGINE"
echo " Project Initializer"
echo "======================================="

ROOT="$(pwd)"

echo
echo "[1/6] Creating directory tree..."

mkdir -p \
.github/workflows \
.github/ISSUE_TEMPLATE \
assets/archive \
assets/audio/ambient \
assets/audio/music \
assets/audio/sfx \
assets/fonts \
assets/icons \
assets/palettes \
assets/shaders \
assets/textures \
docs/architecture \
docs/gameplay \
docs/lore \
docs/renderer \
docs/roadmap \
engine/src \
engine/src/app \
engine/src/audio \
engine/src/core \
engine/src/demon \
engine/src/fractal \
engine/src/gameplay \
engine/src/integration \
engine/src/lore \
engine/src/mining \
engine/src/network \
engine/src/renderer \
engine/src/terminal \
engine/src/ui \
engine/src/world \
saves \
shaders \
tests/fractal \
tests/gameplay \
tests/renderer \
tools/palette-generator \
tools/seed-viewer \
tools/shader-preview

echo "[2/6] Creating Rust module files..."

touch \
engine/src/main.rs \
engine/src/app/mod.rs \
engine/src/app/app.rs \
engine/src/app/state.rs \
engine/src/core/mod.rs \
engine/src/core/config.rs \
engine/src/core/error.rs \
engine/src/core/event.rs \
engine/src/core/logger.rs \
engine/src/core/random.rs \
engine/src/core/seed.rs \
engine/src/core/time.rs \
engine/src/core/version.rs \
engine/src/renderer/mod.rs \
engine/src/renderer/context.rs \
engine/src/renderer/pipeline.rs \
engine/src/renderer/camera.rs \
engine/src/renderer/uniforms.rs \
engine/src/renderer/viewport.rs \
engine/src/fractal/mod.rs \
engine/src/fractal/formulas.rs \
engine/src/fractal/mandelbrot.rs \
engine/src/fractal/julia.rs \
engine/src/fractal/burning_ship.rs \
engine/src/fractal/distance.rs \
engine/src/fractal/iterations.rs \
engine/src/world/mod.rs \
engine/src/world/world.rs \
engine/src/world/biome.rs \
engine/src/world/dimension.rs \
engine/src/world/environment.rs \
engine/src/demon/mod.rs \
engine/src/demon/seed.rs \
engine/src/demon/generator.rs \
engine/src/demon/boss.rs \
engine/src/demon/ecology.rs \
engine/src/demon/mutation.rs \
engine/src/mining/mod.rs \
engine/src/mining/depth.rs \
engine/src/mining/scanner.rs \
engine/src/mining/resources.rs \
engine/src/mining/anomalies.rs \
engine/src/gameplay/mod.rs \
engine/src/gameplay/player.rs \
engine/src/gameplay/combat.rs \
engine/src/gameplay/inventory.rs \
engine/src/gameplay/progression.rs \
engine/src/gameplay/weapon.rs \
engine/src/audio/mod.rs \
engine/src/audio/analyser.rs \
engine/src/audio/fft.rs \
engine/src/audio/music.rs \
engine/src/audio/reactive.rs \
engine/src/terminal/mod.rs \
engine/src/terminal/commands.rs \
engine/src/terminal/console.rs \
engine/src/terminal/history.rs \
engine/src/network/mod.rs \
engine/src/network/client.rs \
engine/src/network/server.rs \
engine/src/network/protocol.rs \
engine/src/network/shared_world.rs \
engine/src/lore/mod.rs \
engine/src/lore/generator.rs \
engine/src/lore/templates.rs \
engine/src/lore/encyclopedia.rs \
engine/src/integration/mod.rs \
engine/src/integration/api.rs \
engine/src/integration/pumpfun.rs \
engine/src/ui/mod.rs \
engine/src/ui/hud.rs \
engine/src/ui/overlay.rs \
engine/src/ui/terminal.rs

echo "[3/6] Creating shader files..."

touch \
shaders/fractal.wgsl \
shaders/postprocess.wgsl \
shaders/bloom.wgsl \
shaders/lighting.wgsl \
shaders/crt.wgsl \
shaders/doom_palette.wgsl

echo "[4/6] Creating project files..."

touch \
README.md \
LICENSE \
.editorconfig \
.gitignore \
rustfmt.toml \
clippy.toml

echo "[5/6] Creating .gitkeep placeholders..."

find assets docs saves tests tools -type d -empty -exec touch {}/.gitkeep \;

echo "[6/6] Done."

echo
echo "Project structure successfully initialized."
