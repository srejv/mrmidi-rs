// dm = { uniforms: { tDiffuse: { type: "t", value: null }, dots: { type: "f", value: 40 }, size: { type: "f", value: .3 }, blur: { type: "f", value: .3 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

// uniform sampler2D tDiffuse; 
// uniform float dots; 
// uniform float size; 
// uniform float blur; 
 
// varying vec2 vUv; 
 
vec4 dot_matrix(sampler2D channel, vec2 vUv, float dots, float size, float blur) {
    float dotSize = 1.0/dots; 
    vec2 samplePos = vUv - mod(vUv, dotSize) + 0.5 * dotSize; 
    float distanceFromSamplePoint = distance(samplePos, vUv);  
    vec4 col = texture2D(channel, samplePos); 
    return mix(col, vec4(0.0), smoothstep(dotSize * size, dotSize *(size + blur), distanceFromSamplePoint));  
} 
