# Demo

This document describes the included **Hardline Demo Scene**, what it demonstrates, and how its supporting runtime components work.

The demo exists to show how **Summon Style: Hardline** is used in a real scene, driven by runtime logic rather than editor-only configuration.

---

## Demo Scene

Open:

`Assets/SmnStyleHardline/Demo/Scenes/Demo_Island01`


This scene contains a small stylized island environment with:

- Terrain, cliffs, foliage, and props  
- Water, skybox, clouds, and particles  
- Animated characters and ambient wildlife  
- A time-of-day driven style and environment system  

Everything in the scene is rendered using Hardline shaders and controlled by a single `SSHStyleManager`.

---

## What The Demo Demonstrates

The demo illustrates:

- How one style system controls all scene visuals  
- How style presets can be blended at runtime  
- How sky, shadows, ambient light, and outlines are synchronized  
- How gameplay systems interact with Hardline through its API  

The scene blends between four `SSHStyleAsset` presets:
- Sunrise  
- Day  
- Sunset  
- Night  

These presets are blended and applied through the Hardline API.

---

## Using The Demo

When the scene is running:

### Time of Day

At the top of the screen is a time slider.

- Dragging the slider changes the current time  
- The play button animates the cycle  

You will see:
- Shadow color and strength update  
- Ambient environment light change  
- Skybox gradients and stars shift  
- Outline settings update  

These changes are driven by style preset blending.

---

### Camera

The demo camera supports two modes.

#### Freecam
- Right Mouse + Mouse: Look  
- WASD / QE: Move  
- Scroll Wheel: Adjust movement speed  

This mode allows free exploration.

#### Cinematic
The camera moves between preset transforms placed around the scene.

Use:
- A / D or the UI arrows to step between shots  

This mode is intended for evaluating composition and silhouette.

---

### UI

Press **F4** to toggle the demo UI.

The UI provides:
- Time of day playback  
- Manual time scrubbing  
- Camera mode switching  
- Shot navigation  
- Control hints  

The UI is implemented with Unity UI Toolkit and communicates with the demo scripts.

---

## Demo Runtime Components

The demo scene is driven by a small set of runtime-only components.  
These are not part of the Hardline core system.

---

### SSHDemoDayNightManager

This component demonstrates how to:

- Blend multiple `SSHStyleAsset` presets  
- Evaluate a normalized time-of-day value  
- Apply the resulting values to Hardline through `SSHStyleManager.API`  

It holds four presets:
- Sunrise  
- Day  
- Sunset  
- Night  

A single normalized `cycleValue` controls which presets are blended.  
The script linearly interpolates values and pushes them into:

- Environment  
- Skybox  
- Outline  

using the Hardline API.

This script serves as a reference for integrating:
- Time of day systems  
- Weather systems  
- Mood or biome transitions  

---

### SSHDemoCamera

Provides two camera modes:

- Free-fly camera for scene inspection  
- Cinematic camera that interpolates between preset transforms  

Cinematic camera targets are simple scene objects used as shot markers.

---

### SSHDemoUI

Handles the demo interface.

It:
- Connects UI Toolkit elements to the demo systems  
- Toggles camera modes  
- Controls time-of-day playback and scrubbing  
- Updates icons and information text  
- Shows or hides UI panels  

---

### SSHDemoAnimatorManager

Manages animated characters and props in the demo.

It:
- Starts a configured animation  
- Disables the Animator at distance  
- Disables SkinnedMeshRenderers at farther distances  
- Supports attaching props to animated bones  

This reduces runtime cost while keeping characters animated when nearby.

---

### SSHDemoCheapBoid

Controls simple ambient motion for background entities.

It:
- Simulates a group of transforms inside a bounded volume  
- Uses deterministic noise for stable movement  
- Keeps motion planar and stylized  

Used for birds, fish, and other ambient motion.

---

## Demo Assets

The demo uses:

- Sample materials  
- Sample meshes and prefabs  
- UI sprites for controls and indicators  

These assets exist to demonstrate shader usage, lighting response, and composition.  
They are not intended to be production-ready content.

---

## Summary

The demo shows how:

- One `SSHStyleManager` controls the scene style
- Style presets are blended and applied at runtime  
- Gameplay-style scripts drive visual changes  

`SSHDemoDayNightManager` is the primary reference for applying dynamic style control through the Hardline API.
