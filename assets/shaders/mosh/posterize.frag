// Mm = { uniforms: { tDiffuse: { type: "t", value: null }, levels: { type: "f", value: 4 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float levels; 
varying vec2 vUv; 

void main() { 
    vec4 col = texture2D( tDiffuse, vUv ); 
    gl_FragColor.rgb = floor((col.rgb * levels) + vec3(0.5)) / levels; 
} 
