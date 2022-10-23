// Tm = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 0 }, amount: { type: "f", value: .05 } }, 
// vertexShader: "" + "\t\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float time; 
uniform float amount; 
 
varying vec2 vUv; 
 
float random1d(float n){ 
    return fract(sin(n) * 43758.5453); 
} 
 
void main() { 
    vec2 p = vUv; 
    vec2 offset = (vec2(random1d(time),random1d(time + 999.99)) - 0.5) * amount; 
    p += offset; 
    gl_FragColor = texture2D(tDiffuse, p); 
}
