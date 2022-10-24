// bm = { uniforms: { tDiffuse: { type: "t", value: null }, pixelsX: { type: "f", value: 10 }, pixelsY: { type: "f", value: 10 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

// uniform sampler2D tDiffuse;
// uniform float pixelsX;  
// uniform float pixelsY; 
// varying vec2 vUv;
 
void pixelize(inout vec4 out_color, sampler2D channel, vec2 vUv, float pixelsX, float pixelsY) { 
    vec2 p = vUv; 
    p.x = floor(p.x * pixelsX)/pixelsX + 0.5/pixelsX;  
    p.y = floor(p.y * pixelsY)/pixelsY + 0.5/pixelsY; 
    out_color = texture2D(channel, p); 
}
