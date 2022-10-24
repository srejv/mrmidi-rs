// xm = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 1 }, speed: { type: "f", value: .5 }, scale: { type: "f", value: .5 }, amount: { type: "f", value: .5 } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

// uniform sampler2D tDiffuse; 
// uniform float time;  
// uniform float scale;  
// uniform float amount; 
// uniform float speed; 
// varying vec2 vUv; 
 
// ${um} : include noise! 

float getNoise(vec2 uv, float t) { 
    //generate multi-octave noise based on uv position and time 
    //move noise  over time 
    //scale noise position relative to center 
    uv -= 0.5;  
    //octave 1 
    float scl = 4.0 * scale; 
    float noise = noise2d( vec2(uv.x * scl ,uv.y * scl - t * speed )); 
    //octave 2 
    scl = 16.0 * scale; 
    noise += noise2d( vec2(uv.x * scl + t* speed ,uv.y * scl )) * 0.2 ; 
    //octave 3 
    scl = 26.0 * scale; 
    noise += noise2d( vec2(uv.x * scl + t* speed ,uv.y * scl )) * 0.2 ; 
    return noise; 
} 
 
void noise_sampler(inout vec4 out_color, sampler2D channel, vec2 vUv, float scale, float amount, float speed) { 
    vec2 uv = vUv; 
    float noise = getNoise(uv, iTime * 24.0); 
    vec2 noiseUv = uv + amount * noise; 
    // wrap 
    noiseUv = fract(noiseUv); 
    out_color = texture2D(channel, noiseUv); 
}
