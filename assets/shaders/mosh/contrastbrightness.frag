// Rm = { uniforms: { tDiffuse: { value: null }, brightness: { value: 0 }, contrast: { value: 0 } }, 
/*vertexShader: [
    "varying vec2 vUv;", 
"void main() {", 
"vUv = uv;", 
"gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", 
"}"*/

// uniform sampler2D tDiffuse;
// uniform float brightness;
// uniform float contrast;
// varying vec2 vUv;

// Maybe the sampling should appear before, so it's stackable.
void contrast_brightness_filter(inout vec4 out_color, float brightness, float contrast) {
    // out_color = texture2D( tDiffuse, vUv );
    out_color.rgb += brightness;
    if (contrast > 0.0) 
    {
        out_color.rgb = (out_color.rgb - 0.5) / (1.0 - contrast) + 0.5;
    } 
    else 
    {
        out_color.rgb = (out_color.rgb - 0.5) * (1.0 + contrast) + 0.5;
    }
}
