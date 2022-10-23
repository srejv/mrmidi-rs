// const hm = 
// { uniforms: 
// { 
//  tDiffuse: { type: "t", value: null }, 
//  time: { type: "f", value: 0 }, 
// distortion: { type: "f", value: 3 }, 
// distortion2: { type: "f", value: 5 }, 
// speed: { type: "f", value: .116 }, 
// rollSpeed: { type: "f", value: .05 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float time; 
uniform float distortion;  
uniform float distortion2; 
uniform float speed; 
uniform float rollSpeed; 
varying vec2 vUv; 
 
// ${um}  // Include noise.frag

void main() { 
    vec2 p = vUv; 
    float ty = time * speed * 17.346; 
    float yt = p.y - ty; 
 
    //thick distortion 
    float offset = noise2d(vec2(yt*3.0,0.0))*0.2; 
    offset = offset*distortion * offset*distortion * offset; 
    //fine distortion 
    offset += noise2d(vec2(yt*50.0,0.0))*distortion2*0.002; 
     
    //combine distortion on X with roll on Y 
    gl_FragColor = texture2D(tDiffuse,  vec2(fract(p.x + offset),fract(p.y - time * rollSpeed) )); 
} 
