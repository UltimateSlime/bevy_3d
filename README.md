# Bevy 3D character controller with animation (Rust)

3D open-world game in Rust + Bevy 0.18 (WIP)

![demo](docs/images/demo.gif)

## Controls

| Key | Action |
|---|---|
| WASD | Move (aligned to camera direction) |
| Space | Jump (ground detection via shape cast) |
| Mouse | Rotate camera |
| Scroll | Zoom in/out (TPS only) |
| V | Toggle TPS / FPS camera |
| Left Click | Lock cursor to window |
| Esc | Quit |

## Current Features

### Camera
- **TPS mode** (default): Third-person view with scroll zoom
  - Camera collision (prevents clipping into walls and ground)
  - Free rotation via Yaw/Pitch
- **FPS mode**: First-person view from the player's eye level
- Auto cursor unlock on Alt+Tab

### Player
- Capsule collider with 3D character model (GLB from Mixamo)
- `RigidBody::Dynamic` + `LockedAxes::ROTATION_LOCKED`
- Ground detection via shape cast for jump control
- Player rotates towards movement direction
- `PlayerState` component for state management (Idle / Walking / Jumping)
- Animations: Idle / Walk / Jump

## Known Issues
- Animation desync when hitting walls
- Jump animation needs improvement

## File Structure

```
src/
├── main.rs      # App initialization and plugin registration
├── world.rs     # Ground, buildings, lights, skybox
├── player.rs    # Player, movement, jump, animations, PlayerState
└── camera.rs    # Camera modes, rotation, collision, cursor lock
assets/
├── textures/
│   └── Ryfjallet_cubemap.png
└── models/
    └── player.glb  # Merged idle/walk/jump animations (Mixamo)
docs/
└── images/
    └── demo.gif
```

## Dependencies
- [Bevy 0.18](https://bevyengine.org/)
- [avian3d 0.5](https://github.com/Jondolf/avian)
```