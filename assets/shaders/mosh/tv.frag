// Cm = { uniforms: { tDiffuse: { type: "t", value: null }, slices: { type: "f", value: 10 }, offset: { type: "f", value: .3 }, speedH: { type: "f", value: .5 }, speedV: { type: "f", value: 1 }, time: { type: "f", value: 0 } }, 
// vertexShader: "" + "\t\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float slices;  
uniform float offset; 
uniform float time; 
uniform float speedV;  
uniform float speedH; 
varying vec2 vUv; 
 
float steppedVal(float v, float steps){ 
    return floor(v*steps)/steps; 
} 

//RANDOM  
//1D 
//returns 0 - 1 
float random1d(float n){ 
    return fract(sin(n) * 43758.5453); 
} 
 
//returns 0 - 1 
float noise1d(float p){  
    float fl = floor(p);  
    float fc = fract(p); 
    return mix(random1d(fl), random1d(fl + 1.0), fc); 
}
  
const float TWO_PI = 6.283185307179586; 

void some_tv(inout vec4 out_color, sampler2D channel, vec2 vUv,float slices, float offset, float speedV, float speedH) { 
    vec2 uv = vUv; 
    //variable width strips 
    float n = noise1d(uv.y * slices + iTime * speedV * 3.0); 
    float ns = steppedVal(fract(n  ),slices) + 2.0; 
    
    float nsr = random1d(ns); 
    vec2 uvn = uv; 
    uvn.x += nsr * sin(iTime * TWO_PI + nsr * 20.0) * offset; 
    out_color = texture2D(channel, uvn); 
} 

