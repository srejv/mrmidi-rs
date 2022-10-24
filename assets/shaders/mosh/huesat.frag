// zm = { uniforms: { tDiffuse: { value: null }, hue: { value: 0 }, saturation: { value: 0 } }, 
// vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 

// uniform sampler2D tDiffuse;
// uniform float hue;
// uniform float saturation;
// varying vec2 vUv;

void hue_saturation(inout vec4 out_color, float hue, float saturation) 
{
    // out_color = texture2D( tDiffuse, vUv );
    float angle = hue * 3.14159265;
    float s = sin(angle), c = cos(angle);
    vec3 weights = (vec3(2.0 * c, -sqrt(3.0) * s - c, sqrt(3.0) * s - c) + 1.0) / 3.0;
    float len = length(out_color.rgb);
    out_color.rgb = vec3(dot(out_color.rgb, weights.xyz), dot(out_color.rgb, weights.zxy),  dot(out_color.rgb, weights.yzx) );
    float average = (out_color.r + out_color.g + out_color.b) / 3.0;

    if (saturation > 0.0) 
    {
        out_color.rgb += (average - out_color.rgb) * (1.0 - 1.0 / (1.001 - saturation));
    } 
    else 
    {
        out_color.rgb += (average - out_color.rgb) * (-saturation);
    }
}
