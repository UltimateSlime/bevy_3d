# bevy_3d

A 3D game in development using Rust + Bevy 0.18. A learning project aimed at building a Skyrim-style open-world MMO.

## Environment

- Rust 1.94.1
- Bevy 0.18
- avian3d 0.5
- OS: Windows 11 / GTX 1080 Ti

## Controls

| Key | Action |
|---|---|
| WASD | Move (aligned to camera direction) |
| Space | Jump (ground detection via ray cast) |
| Mouse | Rotate camera |
| V | Toggle TPS / FPS camera |
| Esc | Quit |

## Current Features

### Camera
- **TPS mode** (default): Third-person view from behind and above the player
  - Camera collision (prevents clipping into walls and ground)
  - Free rotation via Yaw/Pitch
- **FPS mode**: First-person view from the player's eye level

### Player
- Capsule collider
- `RigidBody::Dynamic` + `LockedAxes::ROTATION_LOCKED` (prevents tipping over)
- Ground detection via ray cast for jump control

### World
- Ground: static collider using `Collider::cuboid`
- Buildings: colored box-shaped buildings with random heights (static colliders)
- Skybox: cubemap texture

## Roadmap

- [ ] Mouse cursor lock (confined to window)
- [ ] Animations
- [ ] Server development (Tokio / Axum)
- [ ] MMO features

## File Structure
```
src/
├── main.rs      # App initialization and plugin registration
├── world.rs     # Ground, buildings, lights, skybox, camera
└── player.rs    # Player, movement, jump, camera control
assets/
└── textures/
    └── Ryfjallet_cubemap.png
```