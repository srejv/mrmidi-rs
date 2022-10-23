// bm = { uniforms: { tDiffuse: { type: "t", value: null }, pixelsX: { type: "f", value: 10 }, pixelsY: { type: "f", value: 10 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse;
uniform float pixelsX;  
uniform float pixelsY; 
varying vec2 vUv;
 
void main() { 
    vec2 p = vUv; 
    p.x = floor(p.x * pixelsX)/pixelsX + 0.5/pixelsX;  
    p.y = floor(p.y * pixelsY)/pixelsY + 0.5/pixelsY; 
    gl_FragColor = texture2D(tDiffuse, p); 
} 
