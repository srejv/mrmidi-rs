// Sm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, offset: { type: "f", value: .5 }, time: { type: "f", value: .5 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float amount;  
uniform float offset; 
uniform float time; 
 
varying vec2 vUv; 
 
vec3 rainbow2( in float t ){ 
    vec3 d = vec3(0.0,0.33,0.67);    
    return 0.5 + 0.5*cos( 6.28318*(t+d) ); 
} 
  
void rainboo(inout vec4 out_color, sampler2D channel, vec2 vUv) {
    vec2 p = vUv;  
    vec3 origCol = texture2D( channel, p ).rgb; 
    
    vec2 off = texture2D( channel, p ).rg - 0.5; 
    p += off * offset; 
    vec3 rb = rainbow2( (p.x + p.y + time * 2.0) * 0.5); 
    
    vec3 col = mix(origCol,rb,amount); 
    
    out_color = vec4(col, 1.0); 
} 

