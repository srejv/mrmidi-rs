// fm = { uniforms: { tDiffuse: { value: null }, colLight: {}, colDark: {} }, 
// vertexShader: "" + "\t\t${cm}" + "\t", 

uniform sampler2D tDiffuse;
uniform vec3 colLight;
uniform vec3 colDark;

varying vec2 vUv;

//get float luma from color
float luma(vec3 color) { 
    return dot(color, vec3(0.299, 0.587, 0.114)); 
} 
 
//boost contrast 
vec3 boostContrast(vec3 col, float amount){  
    return  (col - 0.5) / (1.0 - amount) + 0.5; 
}

void main() {
    vec3 col =  texture2D(tDiffuse, vUv).rgb; 
    //col += brightness; 
    //col = boostContrast(col,contrast); 
    col = clamp(col,0.0,1.0); 
    col = mix(colDark,colLight, luma(col)); 
    gl_FragColor = vec4(col,1.0);
}
