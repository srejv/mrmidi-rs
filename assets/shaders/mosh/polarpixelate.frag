// wm = { uniforms: { tDiffuse: { type: "t", value: null }, pixelsX: { type: "f", value: .05 }, pixelsY: { type: "f", value: .05 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

uniform sampler2D tDiffuse; 
uniform float pixelsX;  
uniform float pixelsY; 
 
varying vec2 vUv; 
 
void main() { 
   
    vec2 normCoord = 2.0 * vUv - 1.0; 
    // to polar coords 
    float r = length(normCoord); 
    float phi = atan(normCoord.y, normCoord.x); 
    
    r = r - mod(r, pixelsX) + 0.03; 
    phi = phi - mod(phi, pixelsY); 
    
    normCoord.x = r * cos(phi);  
    normCoord.y = r * sin(phi); 
    vec2 textureCoordinateToUse = normCoord / 2.0 + 0.5; 
    gl_FragColor = texture2D(tDiffuse, textureCoordinateToUse ); 

}
