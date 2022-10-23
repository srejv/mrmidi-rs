// Em = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 0 }, strength: { type: "f", value: .001 }, size: { type: "f", value: 50 }, speed: { type: "f", value: 1 } }, 
// vertexShader: `" + "\t\t${cm}" + "\t`, 

uniform sampler2D tDiffuse; 
uniform float time; 
uniform float strength; 
uniform float size; 
uniform float speed;
varying vec2 vUv;

const float TWO_PI = 6.283185307179586;

void main() {
    vec2 p = -1.0 + 2.0 * vUv;
    float pos = time * TWO_PI + length(p * size);
    gl_FragColor = texture2D(tDiffuse, vUv + strength * vec2(cos(pos), sin(pos)));
} 
