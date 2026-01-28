# Requirements [Check Data]

This document defines the minimum technical requirements needed for Summon Style: Hardline to function correctly inside a Unity project.

---

## Unity Version

Summon Style: Hardline requires a modern Unity editor with current Scriptable Render Pipeline support.

**Supported LTS Versions:** 

| Unity Version | Status |
|---------------|--------|
| 2022 LTS | Supported |
| 2023.x | Supported |
| 2021 LTS | Not supported |
| Older | Not supported |

**All Tested Versions:**
- Unity 2022.3.15f1
- Unity 2023.2.18f1

---

## Render Pipeline (URP)

Hardline only supports the Universal Render Pipeline.

**Supported Render Pipelines:**

| Pipeline | Support |
|--------|--------|
| Universal Render Pipeline (URP) | Required |
| Built-in Render Pipeline | Not supported |
| HDRP | Not supported |

Hardline relies on URP render features, render passes, and shader hooks that do not exist in the Built-in pipeline and are not shared verbatim with the HDRP.

**Supported URP Versions**

| URP Package | Status |
|-------------|--------|
| URP 14+ | Required |
| Below URP 14 | Not supported |

Hardline uses:
- Scriptable Render Features  
- Depth and opaque texture injection  
- Custom outline and skybox passes  

These are only reliable in modern URP versions.

---

## Color Space

| Setting | Requirement |
|--------|-------------|
| Color Space | Linear |

Gamma color space is not supported.  
Hardline’s gradient ramps, ambient light blending, and shadow tinting are authored and evaluated in linear space.

---

## Required URP Settings

The active URP Renderer must have:

- Depth Texture enabled  
- Opaque Texture enabled  

These are required for:
- Outline depth testing  
- Shadow color sampling  
- Environment and skybox lighting

---

## Shader Compatibility

It is highly recommended to use the provided Hardline shaders or expand based off of them for materials used with this system.

 It is technically possible to use Unity’s standard shaders (Lit, URP Unlit, Shader Graph PBR, etc.) but they may produce artifacts and will interfere with outline rendering. Standard shaders will also not get the same environment lighting and shadowing effects as the Hardline shaders. 

---

## Rendering Architecture Assumptions

Hardline assumes:

- A URP camera rendering the scene
- A URP Renderer with the Hardline Render Features installed
- A single active style system (via `SSHStyleManager`) driving all materials

Do not worry about assigning URP Renderers manually unless you have a specific need. It's safe to use the Wizard to set this up the system automatically in the `Installation.md` document.

Camera stacking and multi-renderer setups will currently omit the style system or render in unpredictable ways.
Note, UI is purposefully not changed by the hardline so that designers can decorate UI however they like.

---

## Platform Support

Hardline supports any platform supported by URP that provides:

- Depth textures
- Scriptable Render Features  

WebGL is not supported.

---

## Scripting & Runtime

Hardline requires:

- C# scripting backend  
- Scriptable Render Pipeline assemblies  
- Standard Unity runtime (no stripped render builds)

---

## Validation Checklist

Your project must meet all of the following before installation:

- [ ] Unity 2022+  
- [ ] Universal Render Pipeline active  
- [ ] URP 14+ installed  
- [ ] Linear color space enabled  
- [ ] Depth Texture enabled  
- [ ] Opaque Texture enabled  

If any item is missing, Hardline is not guaranteed to render correctly.

---

If all requirements are met, continue to:

→ [Installation](./Installation.md)
