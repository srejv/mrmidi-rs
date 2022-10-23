# mrmidi-r(eturn)s

Let's go. This time in i rust with better structures.

Stolen input ideas from shadertoy:

4 Sampler channels:
 * Audio
 * Keyboard Input
 * MIDI Input
 * 2D Image Texture
 * Cube Texture
 * Video Texture

But we can also choose to render these on a mesh.

Then we do some mixing in a prost process -> main image function.


4 tracks to set up buffers which are used in post processing.

Question: How to set uniforms actually? :think:


Okay so... design parts?

## The PIPELINE :skalman:

### Editors (data that might be preferable to set up before hand. Also importable?)
* Mesh
 - Static
 - Dynamic
* Texture
 - Static
 - Dynamic?
* Shader (but should be able to edit while using of course, but an external editor to set up exported parameters and stuff might be of help at least in the beginning)
 - Export to Code Tree
 - Export to Shader String

### Inputs
* Audio
    - Raw
    - FFT
    - Envelope Followers (dynamic amount?)
* Midi
    - Velocity
    - CC
* OSC
    - Unknown
* Keyboard
    - PressedLastFrame
    - IsDown
    - ???
* Mouse
    - x, y, isdown, isheld?
* Custom bitset?
    - Some way of customising input protocols maybe, that all the other things are built upon would probably be neat
    - InputBuffers?
    - InputVariables?

### Track (one world)
* Camera - position, rotation, scale (could be fun to LFO I think)
* Meshes -> Shaders (default should usually be enough)
    - Textures.
* Systems? ECS? Maybe easily spawnable entities?
* Particle effects?

Maybe track could also be turned into a webcam? Should try. 

### Mixer (mixing the worlds together)
 * Tracks
 * Single quad rendering
 * Heavy on custom fx pipeline
    - List of effects with enable/disable option like in photomosh?
    - Export variable to Performance/LFO

### Modulation
 - Should be able to modulate anything


## PostFX PhotoMosh Shader

Could be a fun excercise in dynamically building shaders.

### Effects
* Pixelate
    - on
    - Horiz Pixels
    - Vert Pixels
* Polar Pixelate
    - on
    - Radius
    - Segments
* Slices
    - on
    - count
    - offset
    - vertical speed
* Jitter
    - on
    - amount
    - speed
* Melt
    - on
    - amount
    - scale
    - speed
* Wobble
    - on
    - amount
    - size
* Shake
    - on
    - amount
* Edges
    - on
    - amount
    - passthrough
* Solarize
    - on
    - brightness
    - power
    - colorize
* Dot Matrix
    - on
    - count
    - size
    - blue
* Posterize
    - on
    - levels
* DuoTone
    - on
    - light color
    - dark color
    - threshold?
* Bad TV
    - on
    - thick distort
    - fine distort
    - roll speed
* Brightness & Contrast
    - on
    - contrast
    - brightness
* Half Tone
    - on
    - scale
* Linocut
    - on
    - scale
    - noiseScale
    - centerX
* Rainbow
    - on
    - amount
    - offset
* RGB Shift
    - on
    - amount
* Mirror
    - on
    - side
* InstaColor
    - on
    - strength
    - style
* Glow
    - on
    - strength
    - size
    - cut off
* Hue & Saturation
    - on
    - saturation
* Vignette
    - on
    - amount
* Tilt Shift
    - on
    - amount
    - position
* Barrel Blur
    - on
    - amount
* Smear
    - on
    - amount
* Scanlines
    - on
    - count
    - lines count
    - noise amount
    

