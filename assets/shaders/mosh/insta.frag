// _m = { uniforms: { tDiffuse: { type: "t", value: null }, lookupTable: { type: "t", value: null }, strength: { type: "f", value: 1 } }, 
// vertexShader: "" + "\t${cm}" + "\t",

uniform sampler2D tDiffuse; 
uniform sampler2D lookupTable; 
uniform float strength; 
varying vec2 vUv; 

void main() { 
    vec4 col = texture2D( tDiffuse, vUv ); 
    float blueColor = col.b * 63.0; 
    
    vec2 quad1; 
    quad1.y = floor(floor(blueColor) / 8.0); 
    quad1.x = floor(blueColor) - (quad1.y * 8.0); 
    
    vec2 quad2; 
    quad2.y = floor(ceil(blueColor) / 8.0); 
    quad2.x = ceil(blueColor) - (quad2.y * 8.0); 
    
    vec2 texPos1; 
    texPos1.x = (quad1.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.r); 
    texPos1.y = (quad1.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.g); 
    
    //INVERT 
    texPos1.y = 1.0-texPos1.y; 
    
    vec2 texPos2; 
    texPos2.x = (quad2.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.r); 
    texPos2.y = (quad2.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.g); 
    
    //INVERT + texPos2.y = 1.0-texPos2.y; 
    
    vec4 newColor1 = texture2D(lookupTable, texPos1); 
    vec4 newColor2 = texture2D(lookupTable, texPos2); 
    
    vec4 newColor = mix(newColor1, newColor2, fract(blueColor)); 
    
    gl_FragColor = mix(col, vec4(newColor.rgb, col.w), strength); 
}
