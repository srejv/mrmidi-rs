// Mm = { uniforms: { tDiffuse: { type: "t", value: null }, levels: { type: "f", value: 4 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float levels; 
varying vec2 vUv; 

void posterize(inout vec4 out_color, float levels) { 
    out_color.rgb = floor((out_color.rgb * levels) + vec3(0.5)) / levels;
}
