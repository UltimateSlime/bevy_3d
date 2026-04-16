# Bevy 3D character controller with animation (Rust)

3D character controller for Bevy 0.18 + avian3d 0.5 featuring kinematic rigidbody, jump, crouch, dash, wall sliding, and Mixamo animations.

![demo](docs/images/demo.gif)

## Controls

| Key | Action |
|---|---|
| WASD | Move (aligned to camera direction) |
| Space | Jump (ground detection via shape cast) |
| Shift | Dash |
| Ctrl | Crouch (hold) |
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
- `RigidBody::Kinematic` with custom character controller
  - Custom gravity, ground detection, and collision resolution
  - Move-and-slide: wall sliding using normal projection
- Ground detection via shape cast
- Player rotates towards movement direction
- `PlayerState` component for state management (Idle / Walking / Running / Jumping / CrouchIdle / CrouchWalking)
- Crouch: collider resize + camera offset + ceiling check (`can_stand`)
- Animations: Idle / Walk / Run / Jump / CrouchIdle / CrouchWalking

## File Structure

```
src/
├── main.rs      # App initialization and plugin registration
├── world.rs     # Ground, buildings, lights, skybox
├── player.rs    # Player, movement, jump, crouch, animations, PlayerState
└── camera.rs    # Camera modes, rotation, collision, cursor lock
assets/
├── textures/
│   └── Ryfjallet_cubemap.png
└── models/
    └── player.glb  # Merged animations (Mixamo): Idle/Jump/Walk/CrouchIdle/CrouchWalking/Running
docs/
└── images/
    └── demo.gif
```

## Dependencies
- [Bevy 0.18](https://bevyengine.org/)
- [avian3d 0.5](https://github.com/Jondolf/avian)