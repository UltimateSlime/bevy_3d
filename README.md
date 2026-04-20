# Bevy 3D character controller with animation (Rust)

3D character controller for Bevy 0.18 + avian3d 0.5 featuring kinematic rigidbody, jump, crouch, dash, wall sliding, flight, and Mixamo animations.

![demo](docs/images/demo.gif)

## Controls

| Key | Action |
|---|---|
| WASD | Move (aligned to camera direction) |
| Space | Jump |
| Space × 2 | Enter Floating (flight) |
| Shift | Dash / Flying speed boost (in flight) |
| Ctrl | Crouch (hold) / Descend (in flight) |
| Ctrl × 2 | Exit Floating/Flying |
| F | Enter Floating (alternative) |
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
- `PlayerState` component for state management (Idle / Walking / Running / Jumping / Falling / CrouchIdle / CrouchWalking / Floating / Flying)
- Crouch: collider resize + camera offset + ceiling check (`can_stand`)
- Falling: terminal velocity clamp / dive mode (Shift during fall: 3× gravity, higher terminal velocity)
- Flight: Floating (normal speed) and Flying (Shift boost) with full 6DoF movement
- Animations: Idle / Walk / Run / Jump / Falling / CrouchIdle / CrouchWalking / Floating / Flying

## File Structure

```
src/
├── main.rs      # App initialization and plugin registration
├── world.rs     # Ground, buildings, lights, skybox
├── player.rs    # Player, movement, jump, crouch, flight, animations, PlayerState
└── camera.rs    # Camera modes, rotation, collision, cursor lock
assets/
├── textures/
│   └── Ryfjallet_cubemap.png
└── models/
    └── player.glb  # Merged animations (Mixamo): Idle/Jump/Walk/CrouchIdle/CrouchWalking/Running/Floating/Flying/Falling
docs/
└── images/
    └── demo.gif
```

## Dependencies
- [Bevy 0.18](https://bevyengine.org/)
- [avian3d 0.5](https://github.com/Jondolf/avian)