# phase0_scaffold.py
import os
import shutil

print("⛧-Doom-Slayer-⛧ Initialization Phase 0: Foundation Hardening...")

# 1. RF-06: MSRV Pin
with open("rust-toolchain.toml", "w") as f:
    f.write("""[toolchain]
channel = "1.81.0"
components = [ "rustfmt", "clippy" ]
profile = "minimal"
""")

# 2. RF-04: Reconcile saves/
if os.path.exists("saves"):
    shutil.rmtree("saves")
with open(".gitignore", "a") as f:
    f.write("\n# Persistence is handled in Phase 8. Directory removed to avoid confusion.\nsaves/\n")

# 3. §B11: Workspace Restructuring
crates = [
    "abyss-core", "abyss-fractal", "abyss-world", "abyss-sim", 
    "abyss-render", "abyss-audio", "abyss-terminal", "abyss-lore", 
    "abyss-net", "abyss-integration"
]

for c in crates:
    os.makedirs(os.path.join("crates", c, "src"), exist_ok=True)
os.makedirs("app/fractal-doom-abyss-engine/src", exist_ok=True)

def move_module(src_dir, dest_crate):
    dest_src = os.path.join(dest_crate, "src")
    if not os.path.exists(src_dir): return
    
    mod_file = os.path.join(src_dir, "mod.rs")
    lib_file = os.path.join(dest_src, "lib.rs")
    if os.path.exists(mod_file):
        shutil.move(mod_file, lib_file)
        
    for item in os.listdir(src_dir):
        shutil.move(os.path.join(src_dir, item), os.path.join(dest_src, item))
    os.rmdir(src_dir)

move_module("engine/src/core", "crates/abyss-core")
move_module("engine/src/fractal", "crates/abyss-fractal")
move_module("engine/src/world", "crates/abyss-world")
move_module("engine/src/mining", "crates/abyss-world")
move_module("engine/src/demon", "crates/abyss-sim")
move_module("engine/src/gameplay", "crates/abyss-sim")
move_module("engine/src/renderer", "crates/abyss-render")
move_module("engine/src/ui", "crates/abyss-render")
move_module("engine/src/audio", "crates/abyss-audio")
move_module("engine/src/terminal", "crates/abyss-terminal")
move_module("engine/src/lore", "crates/abyss-lore")
move_module("engine/src/network", "crates/abyss-net")
move_module("engine/src/integration", "crates/abyss-integration")

move_module("engine/src/app", "app/fractal-doom-abyss-engine")
if os.path.exists("engine/src/main.rs"):
    shutil.move("engine/src/main.rs", "app/fractal-doom-abyss-engine/src/main.rs")

if os.path.exists("engine"):
    shutil.rmtree("engine")

# 4. Cargo.toml generation
with open("Cargo.toml", "w") as f:
    f.write("""[workspace]
resolver = "2"
members = [
    "crates/abyss-core",
    # The remaining crates are temporarily disabled pending import fixes in Phase 1-2.
    # "crates/abyss-fractal",
    # "crates/abyss-world",
    # "crates/abyss-sim",
    # "crates/abyss-render",
    # "crates/abyss-audio",
    # "crates/abyss-terminal",
    # "crates/abyss-lore",
    # "crates/abyss-net",
    # "crates/abyss-integration",
    # "app/fractal-doom-abyss-engine",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
""")

for c in crates:
    with open(f"crates/{c}/Cargo.toml", "w") as f:
        f.write(f"""[package]
name = "{c}"
version.workspace = true
edition.workspace = true

[dependencies]
""")

with open("app/fractal-doom-abyss-engine/Cargo.toml", "w") as f:
    f.write("""[package]
name = "fractal-doom-abyss-engine"
version.workspace = true
edition.workspace = true

[dependencies]
abyss-core = { path = "../../crates/abyss-core" }
""")

print("✅ The Workspace framework has been successfully created..")