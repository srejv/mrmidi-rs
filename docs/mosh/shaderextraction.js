im.call(this), this.scene = t, this.camera = e, this.overrideMaterial = i, this.clearColor = n, this.clearAlpha = void 0 !== r ? r : 0, this.clear = !0, this.clearDepth = !1, this.needsSwap = !1 }; lm.prototype = Object.assign(Object.create(im.prototype), { constructor: lm, render: function (t, e, i) { var n, r, a = t.autoClear; t.autoClear = !1, this.scene.overrideMaterial = this.overrideMaterial, this.clearColor && (n = t.getClearColor().getHex(), r = t.getClearAlpha(), t.setClearColor(this.clearColor, this.clearAlpha)), this.clearDepth && t.clearDepth(), t.setRenderTarget(this.renderToScreen ? null : i), this.clear && t.clear(t.autoClearColor, t.autoClearDepth, t.autoClearStencil), t.render(this.scene, this.camera), this.clearColor && t.setClearColor(n, r), 
this.scene.overrideMaterial = null, t.autoClear = a } }); 
    
var cm = "" + 
"// precision highp float;" + 
"" + 
"// attribute vec3 position;" + 
"// attribute vec2 uv;" + 
"" + 
"// uniform mat4 modelViewMatrix;" + 
"// uniform mat4 projectionMatrix;" + 
"" + 
"varying vec2 vUv;" + 
"" + 
"void main() {" +
"  vUv = uv;" + 
"  gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1. );" + 
"}"; 
var um = "" + 
"//" + 
"// Description : Array and textureless GLSL 2D simplex noise function." + 
"//      Author : Ian McEwan, Ashima Arts." + 
"//  Maintainer : stegu" + 
"//     Lastmod : 20110822 (ijm)" + 
"//     License : Copyright (C) 2011 Ashima Arts. All rights reserved." + 
"//               Distributed under the MIT License. See LICENSE file." + 
"//               https://github.com/ashima/webgl-noise" + 
"//               https://github.com/stegu/webgl-noise" + 
"//" + "" + "vec3 mod289(vec3 x) {" + 
"  return x - floor(x * (1.0 / 289.0)) * 289.0;" + 
"}" + 
"" + 
"vec2 mod289(vec2 x) {" + 
"  return x - floor(x * (1.0 / 289.0)) * 289.0;" + 
"}" + 
"" + 
"vec3 permute(vec3 x) {" + 
"  return mod289(((x*34.0)+1.0)*x);" + 
"}" + 
"" + 
"float noise2d(vec2 v)" + 
"  {" + 
"  const vec4 C = vec4(0.211324865405187,  // (3.0-sqrt(3.0))/6.0" + 
"                      0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)" + 
"                     -0.577350269189626,  // -1.0 + 2.0 * C.x" + 
"                      0.024390243902439); // 1.0 / 41.0" + "// First corner" + 
"  vec2 i  = floor(v + dot(v, C.yy) );" + 
"  vec2 x0 = v -   i + dot(i, C.xx);" + s
"" + 
"// Other corners" + 
"  vec2 i1;" + 
"  //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0" + 
"  //i1.y = 1.0 - i1.x;" + 
"  i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);" + 
"  // x0 = x0 - 0.0 + 0.0 * C.xx ;" + 
"  // x1 = x0 - i1 + 1.0 * C.xx ;" + 
"  // x2 = x0 - 1.0 + 2.0 * C.xx ;" + 
"  vec4 x12 = x0.xyxy + C.xxzz;" + 
"  x12.xy -= i1;" + 
"" + 
"// Permutations" + 
"  i = mod289(i); // Avoid truncation effects in permutation" + 
"  vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))" + 
"    + i.x + vec3(0.0, i1.x, 1.0 ));" + "" + 
"  vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);" + 
"  m = m*m ;" + "  m = m*m ;" + 
"" + 
"// Gradients: 41 points uniformly over a line, mapped onto a diamond." + 
"// The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)" + 
"" + 
"  vec3 x = 2.0 * fract(p * C.www) - 1.0;" + 
"  vec3 h = abs(x) - 0.5;" + 
"  vec3 ox = floor(x + 0.5);" + 
"  vec3 a0 = x - ox;" + 
"" +
"// Normalise gradients implicitly by scaling m" + 
"// Approximation of: m *= inversesqrt( a0*a0 + h*h );" + 
"  m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );" + 
"" + 
"// Compute final noise value at P" + 
"  vec3 g;" + 
"  g.x  = a0.x  * x0.x  + h.x  * x0.y;" + 
"  g.yz = a0.yz * x12.xz + h.yz * x12.yw;" + 
"  return 130.0 * dot(m, g);" + 
"}"; 
const hm = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 0 }, distortion: { type: "f", value: 3 }, distortion2: { type: "f", value: 5 }, speed: { type: "f", value: .116 }, rollSpeed: { type: "f", value: .05 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float time;" + 
"\tuniform float distortion;" + 
"\tuniform float distortion2;" + 
"\tuniform float speed;" + 
"\tuniform float rollSpeed;" + 
"\tvarying vec2 vUv;" + 
"\t" + 
"\t${um}" + 
"" + 
"\tvoid main() {" + 
"" + 
"\t\tvec2 p = vUv;" + 
"\t\tfloat ty = time * speed * 17.346;" + 
"\t\tfloat yt = p.y - ty;" + 
"" + 
"\t\t//thick distortion" + 
"\t\tfloat offset = noise2d(vec2(yt*3.0,0.0))*0.2;" + 
"\t\toffset = offset*distortion * offset*distortion * offset;" + 
"\t\t//fine distortion" + 
"\t\toffset += noise2d(vec2(yt*50.0,0.0))*distortion2*0.002;" + 
"\t\t" + 
"\t\t//combine distortion on X with roll on Y" + 
"\t\tgl_FragColor = texture2D(tDiffuse,  vec2(fract(p.x + offset),fract(p.y - time * rollSpeed) ));" + 
"" + 
"\t}" + "" }, 
pm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, time: { type: "f", value: 0 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float amount;" + 
"\tuniform float time;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tconst int num_iter = 16;" + 
"\tconst float reci_num_iter_f = 1.0 / float(num_iter);" + 
"\tconst float gamma = 2.2;" + 
"\tconst float MAX_DIST_PX = 200.0;" + 
"" + 
"\tvec2 barrelDistortion( vec2 p, vec2 amt )" + 
"\t{" + 
"\t\tp = 2.0*p-1.0;" + 
"\t\t//float BarrelPower = 1.125;" + 
"\t\tconst float maxBarrelPower = 3.0;" + 
"\t\tfloat theta  = atan(p.y, p.x);" + 
"\t\tfloat radius = length(p);" + 
"\t\tradius = pow(radius, 1.0 + maxBarrelPower * amt.x);" + 
"\t\tp.x = radius * cos(theta);" + 
"\t\tp.y = radius * sin(theta);" + 
"\t\treturn 0.5 * ( p + 1.0 );" + 
"\t}" + 
"" + 
"\tfloat sat( float t )" + 
"\t{" + 
"\t\treturn clamp( t, 0.0, 1.0 );" + 
"\t}" + 
"" +
 "\tfloat linterp( float t ) {" + 
 "\t\treturn sat( 1.0 - abs( 2.0*t - 1.0 ) );" + 
 "\t}" + 
 "" + 
 "\tfloat remap( float t, float a, float b ) {" + 
 "\t\treturn sat( (t - a) / (b - a) );" + 
 "\t}" + 
 "" + 
 "\tvec3 spectrum_offset( float t ) {" + 
 "\t\tvec3 ret;" + 
 "\t\tfloat lo = step(t,0.5);" + 
 "\t\tfloat hi = 1.0-lo;" + 
 "\t\tfloat w = linterp( remap( t, 1.0/6.0, 5.0/6.0 ) );" + 
 "\t\tret = vec3(lo,1.0,hi) * vec3(1.0-w, w, 1.0-w);" + 
 "\t" + 
 "\t\treturn pow( ret, vec3(1.0/2.2) );" + 
 "\t}" + "" + 
 "\tfloat nrand( vec2 n )" + 
 "\t{" + 
 "\t\treturn fract(sin(dot(n.xy, vec2(12.9898, 78.233)))* 43758.5453);" + 
 "\t}" + 
 "" + 
 "\tvec3 lin2srgb( vec3 c )" + 
 "\t{" + 
 "\t\treturn pow( c, vec3(gamma) );" + 
 "\t}" + 
 "" + 
 "\tvec3 srgb2lin( vec3 c )" + 
 "\t{" + 
 "\t\treturn pow( c, vec3(1.0/gamma));" + "\t}" + 
 "" + 
 "\tvoid main() {" + 
 "" + 
 "\t\tvec2 uv = vUv;" + 
 "\t\t//resolution independent" + 
 "\t\tvec2 max_distort = vec2(amount); " + 
 "" + 
 "\t\tvec2 oversiz = barrelDistortion( vec2(1,1), max_distort );" + 
 "\t\tuv = 2.0 * uv - 1.0;" + 
 "\t\tuv = uv / (oversiz*oversiz);" + 
 "\t\tuv = 0.5 * uv + 0.5;" + 
 "" + 
 "\t\tvec3 sumcol = vec3(0.0);" + 
 "\t\tvec3 sumw = vec3(0.0);" + 
 "\t\tfloat rnd = nrand( uv + fract(time) );" + 
 "\t\tfor ( int i=0; i<num_iter;++i ){" + 
 "\t\t\tfloat t = (float(i)+rnd) * reci_num_iter_f;" + 
 "\t\t\tvec3 w = spectrum_offset( t );" + 
 "\t\t\tsumw += w;" + 
 "\t\t\tsumcol += w * srgb2lin(texture2D( tDiffuse, barrelDistortion(uv, max_distort*t ) ).rgb);" +
 "\t\t}" + 
 "" + 
 "\t\tsumcol.rgb /= sumw;" + 
 "\t\tvec3 outcol = lin2srgb(sumcol.rgb);" + 
 "\t\toutcol += rnd/255.0;" + 
 "\t\tgl_FragColor = vec4( outcol, 1.0);" + 
 "\t}" + 
 "\t" }, 
dm = { uniforms: { tDiffuse: { type: "t", value: null }, dots: { type: "f", value: 40 }, size: { type: "f", value: .3 }, blur: { type: "f", value: .3 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float dots;" + 
"\tuniform float size;" + 
"\tuniform float blur;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tvoid main() {" + 
"\t\tfloat dotSize = 1.0/dots;" + 
"\t\tvec2 samplePos = vUv - mod(vUv, dotSize) + 0.5 * dotSize;" + 
"\t\tfloat distanceFromSamplePoint = distance(samplePos, vUv);" + 
"\t\tvec4 col = texture2D(tDiffuse, samplePos);" + 
"\t\tgl_FragColor = mix(col, vec4(0.0), smoothstep(dotSize * size, dotSize *(size + blur), distanceFromSamplePoint));" + 
"" + 
"\t}" + 
"\t" }, 
fm = { uniforms: { tDiffuse: { value: null }, colLight: {}, colDark: {} }, 
vertexShader: "" + "\t\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform vec3 colLight;" + 
"\tuniform vec3 colDark;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\t//get float luma from color" + 
"\tfloat luma(vec3 color) {" + 
"\t\treturn dot(color, vec3(0.299, 0.587, 0.114));" + "\t}" + 
"" + 
"\t//boost contrast" + 
"\tvec3 boostContrast(vec3 col, float amount){" + 
"\t\treturn  (col - 0.5) / (1.0 - amount) + 0.5;" + 
"\t}" + 
"" + 
"\tvoid main() {" + 
"\t\tvec3 col =  texture2D(tDiffuse, vUv).rgb;" + 
"\t\t//col += brightness;" + 
"\t\t//col = boostContrast(col,contrast);" + 
"\t\tcol = clamp(col,0.0,1.0);" + 
"\t\tcol = mix(colDark,colLight, luma(col));" + 
"\t\tgl_FragColor = vec4(col,1.0);" + 
"\t}" + 
"" }, 
mm = { uniforms: { tDiffuse: { value: null }, amount: { value: 0 }, passthru: { value: 0 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float amount;" + 
"\tuniform float passthru;" + 
"\tvarying vec2 vUv;" + "" + 
"\tvec2 texel = vec2(1.0 /512.0);" + 
"" + 
"\tmat3 G[2];" + 
"" + 
"\tconst mat3 g0 = mat3( 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0 );" + 
"\tconst mat3 g1 = mat3( 1.0, 0.0, -1.0, 2.0, 0.0, -2.0, 1.0, 0.0, -1.0 );" + 
"" + 
"" + 
"\tvoid main(void)" + 
"\t{" + 
"\t\tmat3 I;" + 
"\t\tfloat cnv[2];" + 
"\t\tvec3 sample;" + 
"" + 
"\t\tG[0] = g0;" + 
"\t\tG[1] = g1;" + 
"" + 
"\t\t/* fetch the 3x3 neighbourhood and use the RGB vectors length as intensity value */" + 
"\t\tfor (float i=0.0; i<3.0; i++)" + 
"\t\tfor (float j=0.0; j<3.0; j++) {" + 
"\t\t\tsample = texture2D( tDiffuse, vUv + texel * vec2(i-1.0,j-1.0) ).rgb;" + 
"\t\t\tI[int(i)][int(j)] = length(sample);" + 
"\t\t}" + 
"" + 
"\t\t/* calculate the convolution values for all the masks */" + 
"\t\tfor (int i=0; i<2; i++) {" + 
"\t\t\tfloat dp3 = dot(G[i][0], I[0]) + dot(G[i][1], I[1]) + dot(G[i][2], I[2]);" + 
"\t\t\tcnv[i] = dp3 * dp3; " + 
"\t\t}" + 
"" + 
"\t\tvec4 orig = texture2D( tDiffuse, vUv);" + 
"" + 
"\t\tgl_FragColor = orig * passthru + vec4(0.5 * sqrt(cnv[0]*cnv[0]+cnv[1]*cnv[1])) * amount;" + 
"\t}" + 
"" },
vm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, speed: { type: "f", value: .5 }, time: { type: "f", value: 0 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tvarying vec2 vUv;" + "\tuniform float amount;" + 
"\tuniform float speed;" + "\tuniform float time;" + 
"" + 
"\tfloat random1d(float n){" + 
"\t\treturn fract(sin(n) * 43758.5453);" + "\t}" + 
"" + 
"\t//2D (returns 0 - 1)" + 
"\tfloat random2d(vec2 n) { " + 
"\t\treturn fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);" + 
"\t}" + 
"" + 
"\tfloat randomRange (in vec2 seed, in float min, in float max) {" + 
"\t\treturn min + random2d(seed) * (max - min);" + 
"\t}" + 
"" + 
"\t// return 1 if v inside 1d range" + 
"\tfloat insideRange(float v, float bottom, float top) {" + 
"\treturn step(bottom, v) - step(top, v);" + 
"\t}" + "" + "\tfloat rand(vec2 co){" + "\t\treturn fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);" + "\t}" + "" + 
"\tvoid main() {" + 
"\t\t" + 
"\t\tvec2 uv = vUv;" + 
"" + 
"\t\tfloat sTime = floor(time * speed * 6.0 * 24.0);" + 
"\t\tvec3 inCol = texture2D(tDiffuse, uv).rgb;" + 
"\t\t" + 
"\t\t//copy orig" + 
"\t\tvec3 outCol = inCol;" + 
"\t\t" + 
"\t\t//randomly offset slices horizontally" + 
"\t\tfloat maxOffset = amount/2.0;" + 
"" + 
"\t\tvec2 uvOff;" + 
"\t\t" + 
"\t\tfor (float i = 0.0; i < 10.0; i += 1.0) {" + 
"" + 
"\t\t\tif (i > 10.0 * amount) break;" + 
"" + 
"\t\t\tfloat sliceY = random2d(vec2(sTime + amount, 2345.0 + float(i)));" + 
"\t\t\tfloat sliceH = random2d(vec2(sTime + amount, 9035.0 + float(i))) * 0.25;" + 
"\t\t\tfloat hOffset = randomRange(vec2(sTime + amount, 9625.0 + float(i)), -maxOffset, maxOffset);" + 
"\t\t\tuvOff = uv;" + "\t\t\tuvOff.x += hOffset;" + 
"\t\t\tvec2 uvOff = fract(uvOff);" + 
"\t\t\tif (insideRange(uv.y, sliceY, fract(sliceY+sliceH)) == 1.0 ){" + 
"\t\t\t\toutCol = texture2D(tDiffuse, uvOff).rgb;" + 
"\t\t\t}" + "\t\t}" + "\t" + 
"\t\t//do color offset - slight shift on one entire channel" + 
"\t\tfloat maxColOffset = amount/6.0;" + 
"\t\tvec2 colOffset = vec2(randomRange(vec2(sTime + amount, 3545.0),-maxColOffset,maxColOffset), randomRange(vec2(sTime , 7205.0),-maxColOffset,maxColOffset));" + 
"" + 
"\t\tuvOff = fract(uv + colOffset);" + 
"\t\t" + 
"\t\t//TODO - use col[1] array access" + 
"\t\tfloat rnd = random2d(vec2(sTime + amount, 9545.0));" + 
"\t\tif (rnd < 0.33){" + 
"\t\t\toutCol.r = texture2D(tDiffuse, uvOff).r;" + 
"\t\t}else if (rnd < 0.66){" + 
"\t\t\toutCol.g = texture2D(tDiffuse, uvOff).g;" + 
"\t\t} else{" + 
"\t\t\toutCol.b = texture2D(tDiffuse, uvOff).b;" + 
"\t\t}" + 
"\t\tgl_FragColor = vec4(outCol,1.0);" + 
"\t}" + 
"\t" }, 
gm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, size: { type: "f", value: 4 }, darkness: { type: "f", value: .1 }, resolution: { type: "v2" } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float size;" + 
"\tuniform float amount;" + 
"\tuniform vec2 resolution;" + 
"\tuniform float darkness;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tvoid main() {" + 
"" + 
"\t\tfloat h = size / resolution.x;" + 
"\t\tfloat v = size / resolution.y;" + 
"\t\t" + 
"\t\tvec4 sum = vec4( 0.0 );" + 
"" + 
"\t\t//H Blur" + 
"\t\tsum += (texture2D( tDiffuse, vec2( vUv.x - 4.0 * h, vUv.y ) )- darkness) * 0.051 ;" +
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x - 3.0 * h, vUv.y ) )- darkness) * 0.0918;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x - 2.0 * h, vUv.y ) )- darkness) * 0.12245;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x - 1.0 * h, vUv.y ) )- darkness) * 0.1531;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y ) )- darkness) * 0.1633;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x + 1.0 * h, vUv.y ) )- darkness) * 0.1531;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x + 2.0 * h, vUv.y ) )- darkness) * 0.12245;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x + 3.0 * h, vUv.y ) )- darkness) * 0.0918;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x + 4.0 * h, vUv.y ) )- darkness) * 0.051;" + 
 "\t\t" + 
 "\t\t//V Blur" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y - 4.0 * v ) )- darkness) * 0.051;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y - 3.0 * v ) )- darkness) * 0.0918;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y - 2.0 * v ) )- darkness) * 0.12245;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y - 1.0 * v ) )- darkness) * 0.1531;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y ) )- darkness) * 0.1633;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y + 1.0 * v ) )- darkness) * 0.1531;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y + 2.0 * v ) )- darkness) * 0.12245;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y + 3.0 * v ) )- darkness) * 0.0918;" + 
 "\t\tsum += (texture2D( tDiffuse, vec2( vUv.x, vUv.y + 4.0 * v ) )- darkness) * 0.051;" + "" + 
 "\t\t//get original pixel color" + 
 "\t\tvec4 base = texture2D( tDiffuse, vUv );" + 
 "\t\t" + 
 "\t\t//Additive Blend" + 
 "\t\tgl_FragColor = base + max(sum,0.0) * amount;" + 
 "\t}" + 
 "\t" }, 
ym = { uniforms: { tDiffuse: { type: "t", value: null }, resolution: { type: "v2" }, scale: { type: "f", value: 0 }, noiseScale: { type: "f", value: .1 }, centerX: { type: "f", value: .5 } }, 
vertexShader: "" + "\t\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform vec2 resolution;" + 
"\tvarying vec2 vUv;" + 
"\tuniform float scale;" + 
"\tuniform float noiseScale;" + 
"\tuniform float centerX;" + 
"" + 
"\tfloat luma(vec3 color) {" + 
"\t\treturn dot(color, vec3(0.299, 0.587, 0.114));" + "\t}" + 
"" + 
"\tvoid main() {" + "" + 
"\t\tvec2 center = vec2( 0.5 );" + 
"\t\tcenter.x = centerX;" + 
"\t\tvec2 uv = vUv;" + 
"" + 
"\t\t//float noiseScale = 0.1;" + 
"\t\tfloat radius = 0.5;" + 
"\t\tvec2 d = uv - center;" + 
"\t\tfloat r = length( d * vec2( 1., resolution.y / resolution.x ) ) * scale;" + 
"\t\tfloat a = atan(d.y,d.x) + noiseScale*(radius-r)/radius;" + 
"\t\tvec2 uvt = center+r*vec2(cos(a),sin(a));" + 
"" + 
"\t\tvec2 uv2 = vUv;" + 
"\t\tfloat c = ( .75 + .25 * sin( uvt.x * 1000. ) );" + 
"\t\tvec4 color = texture2D( tDiffuse, uv2 );" + 
"\t\tfloat l = luma( color.rgb );" + 
"\t\tfloat f = smoothstep( .5 * c, c, l );" + 
"\t\tf = smoothstep( 0., .5, f );" + "" + "\t\tvec3 col = vec3(f);" + 
"" + 
"\t\tgl_FragColor = vec4( col,.0);" + 
"\t}" + 
"\t" }, 
_m = { uniforms: { tDiffuse: { type: "t", value: null }, lookupTable: { type: "t", value: null }, strength: { type: "f", value: 1 } }, 
vertexShader: "" + "\t${cm}" + "\t",
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform sampler2D lookupTable;" + 
"\tuniform float strength;" + 
"\tvarying vec2 vUv;" + 
"" +
"\tvoid main() {" + 
"\t\tvec4 col = texture2D( tDiffuse, vUv );" + 
"\t\tfloat blueColor = col.b * 63.0;" + 
"" + 
"\t\tvec2 quad1;" + 
"\t\tquad1.y = floor(floor(blueColor) / 8.0);" + 
"\t\tquad1.x = floor(blueColor) - (quad1.y * 8.0);" + 
"" + 
"\t\tvec2 quad2;" + 
"\t\tquad2.y = floor(ceil(blueColor) / 8.0);" + 
"\t\tquad2.x = ceil(blueColor) - (quad2.y * 8.0);" + 
"" + 
"\t\tvec2 texPos1;" + 
"\t\ttexPos1.x = (quad1.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.r);" + 
"\t\ttexPos1.y = (quad1.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.g);" + 
"" + 
"\t\t//INVERT" + 
"\t\ttexPos1.y = 1.0-texPos1.y;" + 
"" + 
"\t\tvec2 texPos2;" + 
"\t\ttexPos2.x = (quad2.x * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.r);" + 
"\t\ttexPos2.y = (quad2.y * 0.125) + 0.5/512.0 + ((0.125 - 1.0/512.0) * col.g);" + 
"" + 
"\t\t//INVERT" + "\t\ttexPos2.y = 1.0-texPos2.y;" + 
"" + 
"\t\tvec4 newColor1 = texture2D(lookupTable, texPos1);" + 
"\t\tvec4 newColor2 = texture2D(lookupTable, texPos2);" + 
"" + 
"\t\tvec4 newColor = mix(newColor1, newColor2, fract(blueColor));" + 
"" + 
"\t\tgl_FragColor = mix(col, vec4(newColor.rgb, col.w), strength);" + "\t}" + "" }, 
xm = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 1 }, speed: { type: "f", value: .5 }, scale: { type: "f", value: .5 }, amount: { type: "f", value: .5 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float time;" + 
"\tuniform float scale;" + 
"\tuniform float amount;" + 
"\tuniform float speed;" + 
"\tvarying vec2 vUv;" + 
"" + 
"\t${um}" + "" + 
"\tfloat getNoise(vec2 uv, float t){" + 
"\t\t//generate multi-octave noise based on uv position and time" + 
"\t\t//move noise  over time" + 
"\t\t//scale noise position relative to center" + 
"\t\tuv -= 0.5;" + 
"\t\t//octave 1" + 
"\t\tfloat scl = 4.0 * scale;" + 
"\t\tfloat noise = noise2d( vec2(uv.x * scl ,uv.y * scl - t * speed ));" + 
"\t\t//octave 2" + 
"\t\tscl = 16.0 * scale;" + 
"\t\tnoise += noise2d( vec2(uv.x * scl + t* speed ,uv.y * scl )) * 0.2 ;" + 
"\t\t//octave 3" + 
"\t\tscl = 26.0 * scale;" + 
"\t\tnoise += noise2d( vec2(uv.x * scl + t* speed ,uv.y * scl )) * 0.2 ;" + 
"\t\treturn noise;" + 
"\t}" + 
"" + 
"\tvoid main() {" + 
"\t\tvec2 uv = vUv;" + 
"\t\tfloat noise = getNoise(uv, time * 24.0);" + 
"\t\tvec2 noiseUv = uv + amount * noise;" + 
"\t\t//wrap" + 
"\t\tnoiseUv = fract(noiseUv);" + 
"\t\tgl_FragColor = texture2D(tDiffuse,noiseUv);" + 
"\t}" + "" }, 
bm = { uniforms: { tDiffuse: { type: "t", value: null }, pixelsX: { type: "f", value: 10 }, pixelsY: { type: "f", value: 10 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float pixelsX;" + 
"\tuniform float pixelsY;" + 
"\tvarying vec2 vUv;" + "" + 
"\tvoid main() {" +
"" + 
"\t\tvec2 p = vUv;" + 
"\t\tp.x = floor(p.x * pixelsX)/pixelsX + 0.5/pixelsX;" + 
"\t\tp.y = floor(p.y * pixelsY)/pixelsY + 0.5/pixelsY;" + 
"\t\tgl_FragColor = texture2D(tDiffuse, p);" + 
"" + "\t}" + "" }, 
wm = { uniforms: { tDiffuse: { type: "t", value: null }, pixelsX: { type: "f", value: .05 }, pixelsY: { type: "f", value: .05 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float pixelsX;" + 
"\tuniform float pixelsY;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tvoid main() {" + 
"" + 
"\t\tvec2 normCoord = 2.0 * vUv - 1.0;" + 
"\t\t// to polar coords" + 
"\t\tfloat r = length(normCoord); " +
"\t\tfloat phi = atan(normCoord.y, normCoord.x);" + 
"\t\t\t" + 
"\t\tr = r - mod(r, pixelsX) + 0.03;" + 
"\t\tphi = phi - mod(phi, pixelsY);" + 
"\t\t\t" + 
"\t\tnormCoord.x = r * cos(phi);" + 
"\t\tnormCoord.y = r * sin(phi);" + 
"\t\tvec2 textureCoordinateToUse = normCoord / 2.0 + 0.5;" + 
"\t\tgl_FragColor = texture2D(tDiffuse, textureCoordinateToUse );" + 
"\t" + 
"\t}" + "" }, 
Mm = { uniforms: { tDiffuse: { type: "t", value: null }, levels: { type: "f", value: 4 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float levels;" + 
"\tvarying vec2 vUv;" + "" + 
"\tvoid main() {" + 
"\t\tvec4 col = texture2D( tDiffuse, vUv );" + 
"\t\tgl_FragColor.rgb = floor((col.rgb * levels) + vec3(0.5)) / levels;" + 
"\t}" + "" }, 
Sm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, offset: { type: "f", value: .5 }, time: { type: "f", value: .5 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float amount;" + 
"\tuniform float offset;" + 
"\tuniform float time;" + "" + 
"\tvarying vec2 vUv;" + "" + 
"\tvec3 rainbow2( in float t ){" + 
"\t\tvec3 d = vec3(0.0,0.33,0.67);   " + 
"\t\treturn 0.5 + 0.5*cos( 6.28318*(t+d) );" + 
"\t}" + 
"" + 
"\tvoid main() {" + 
"\t\tvec2 p = vUv;" + 
"\t\tvec3 origCol = texture2D( tDiffuse, p ).rgb;" + 
"" + 
"\t\tvec2 off = texture2D( tDiffuse, p ).rg - 0.5;" + 
"\t\tp += off * offset;" + 
"\t\tvec3 rb = rainbow2( (p.x + p.y + time * 2.0) * 0.5);" + 
"" + 
"\t\tvec3 col = mix(origCol,rb,amount);" + 
"" + 
"\t\tgl_FragColor = vec4(col, 1.0);" + 
"" + 
"\t}" + "" }, 
Am = { uniforms: { tDiffuse: { value: null }, time: { value: 0 }, noiseAmount: { value: .5 }, linesAmount: { value: .05 }, count: { value: 4096 }, height: { value: 4096 } }, 
vertexShader: "" + "\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float time;" + 
"\tuniform float count;" + 
"\tuniform float noiseAmount;" + 
"\tuniform float linesAmount;" + 
"\tuniform float height;" + 
"" + 
"\tvarying vec2 vUv;" + 
"" + 
"\t#define PI 3.14159265359" + 
"" + 
"\thighp float rand( const in vec2 uv ) {" + 
"\t\tconst highp float a = 12.9898, b = 78.233, c = 43758.5453;" + 
"\t\thighp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );" + 
"\t\treturn fract(sin(sn) * c);" + 
"\t}" + 
"" + 
"\tvoid main() {" + 
"" + 
"\t\t// sample the source" + 
"\t\tvec4 cTextureScreen = texture2D( tDiffuse, vUv );" + 
"\t\t" + 
"\t\t// add noise" + 
"\t\tfloat dx = rand( vUv + time );" + 
"\t\tvec3 cResult = cTextureScreen.rgb * dx * noiseAmount;" + 
"\t\t" + 
"\t\t// add scanlines" + "\t\tfloat lineAmount = height * 1.8 * count;" + 
"\t\tvec2 sc = vec2( sin( vUv.y * lineAmount), cos( vUv.y * lineAmount) );" + 
"\t\tcResult += cTextureScreen.rgb * vec3( sc.x, sc.y, sc.x ) * linesAmount;" + 
"" + 
"\t\t// interpolate between source and result by intensity" + 
"\t\tcResult = cTextureScreen.rgb + ( cResult );" + 
"" + 
"\t\tgl_FragColor =  vec4( cResult, cTextureScreen.a );" + 
"\t}" + 
"" }, 
Tm = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 0 }, amount: { type: "f", value: .05 } }, 
vertexShader: "" + "\t\t${cm}" + "\t", 
fragmentShader: "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float time;" + 
"\tuniform float amount;" + "" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tfloat random1d(float n){" + 
"\t\treturn fract(sin(n) * 43758.5453);" + 
"\t}" + "" + 
"\tvoid main() {" + 
"\t\tvec2 p = vUv;" + 
"\t\tvec2 offset = (vec2(random1d(time),random1d(time + 999.99)) - 0.5) * amount;" + 
"\t\tp += offset;" + 
"\t\tgl_FragColor = texture2D(tDiffuse, p);" + 
"\t}" + 
"" }, 
Cm = { uniforms: { tDiffuse: { type: "t", value: null }, slices: { type: "f", value: 10 }, offset: { type: "f", value: .3 }, speedH: { type: "f", value: .5 }, speedV: { type: "f", value: 1 }, time: { type: "f", value: 0 } }, 
vertexShader: "" + "\t\t${cm}" + "\t", 
fragmentShader: "" + "" + 
"\tuniform sampler2D tDiffuse;" + 
"\tuniform float slices;" + 
"\tuniform float offset;" + 
"\tuniform float time;" + 
"\tuniform float speedV;" + 
"\tuniform float speedH;" + 
"\tvarying vec2 vUv;" + 
"" + 
"\tfloat steppedVal(float v, float steps){" + 
"\t\treturn floor(v*steps)/steps;" + 
"\t}" + 
"" + 
"\t//RANDOM " + 
"\t//1D" + "\t//returns 0 - 1" + 
"\tfloat random1d(float n){" + "\t\treturn fract(sin(n) * 43758.5453);" + 
"\t}" + 
"" + 
"\t//returns 0 - 1" + 
"\tfloat noise1d(float p){" + 
"\t\tfloat fl = floor(p);" + 
"\t\tfloat fc = fract(p);" + 
"\t\treturn mix(random1d(fl), random1d(fl + 1.0), fc);" + 
"\t}" +
 "" + 
"\tconst float TWO_PI = 6.283185307179586;" + "" + 
"\tvoid main() {" + 
"\t\tvec2 uv = vUv;" + "\t\t//variable width strips" + 
"\t\tfloat n = noise1d(uv.y * slices + time * speedV * 3.0);" + 
"\t\tfloat ns = steppedVal(fract(n  ),slices) + 2.0;" + "\t\t" + 
"\t\tfloat nsr = random1d(ns);" + 
"\t\tvec2 uvn = uv;" + 
"\t\tuvn.x += nsr * sin(time * TWO_PI + nsr * 20.0) * offset;" + 
"\t\tgl_FragColor = texture2D(tDiffuse, uvn);" + 
"\t}" + 
"" }, 
Lm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, time: { type: "f", value: .5 } }, 
vertexShader: "" + "\t\t${cm}" + "\t", 
fragmentShader: "" +
 "\tconst float TWO_PI = 6.283185307179586;" + 
 "" + 
 "\tuniform sampler2D tDiffuse;" + 
 "\tuniform float amount;" + 
 "\tuniform float time;" + 
 "" +
  "\tvarying vec2 vUv;" + "" + "\tvec2 rotate2D(vec2 position, float theta){" + 
  "\t\tmat2 m = mat2( cos(theta), -sin(theta), sin(theta), cos(theta) );" + "\t\treturn m * position;" + "\t}" + "" + "\tvoid main() {" + "\t\tvec2 p = vUv;" + "\t\t//Displace image by its own rg channel" + "\t\tvec2 sPos = vUv;" + "\t\tvec2 off = texture2D( tDiffuse, sPos ).rg - 0.5;" + "" + "\t\t//rotate" + "\t\tfloat ang = time * TWO_PI;" 
+ "\t\toff = rotate2D(off,ang);" + 
"\t\tp += off * amount;" + 
"\t\tvec4 col = texture2D(tDiffuse,p);" + 
"\t\tgl_FragColor = col;" + "\t}" + "" }, 
Pm = { uniforms: { tDiffuse: { type: "t", value: null }, centerBrightness: { type: "f", value: .5 }, powerCurve: { type: "f", value: 2 }, colorize: { type: "f", value: .1 } }, 
vertexShader: `" + "\t\t${cm}" + "\t`, 
fragmentShader: 
"uniform sampler2D tDiffuse;"+
"uniform float centerBrightness;"+
"uniform float powerCurve;"+
"uniform float colorize;"+
"varying vec2 vUv;"+
"vec3 rgb2hsv(vec3 c) {"+
"vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);"+
"vec4 p = c.g < c.b ? vec4(c.bg, K.wz) : vec4(c.gb, K.xy);"+
"vec4 q = c.r < p.x ? vec4(p.xyw, c.r) : vec4(c.r, p.yzx);"+
"float d = q.x - min(q.w, q.y);"+
"float e = 1.0e-10;"+
"return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);"+
"}"+
"vec3 hsv2rgb(vec3 c){"+
"vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);"+
"vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);"+
"return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);"+
"}"+
"void main() {"+
"vec3 origCol = texture2D( tDiffuse, vUv ).rgb;"+
//\tconvert to HSV
"vec3 hslColor = rgb2hsv(origCol);"+
"vec3 outColor = hslColor;"+

//\tadjust the brightness curve" + "
"outColor.b = pow(outColor.b, powerCurve);"+
"outColor.b = (outColor.b < centerBrightness) ? (1.0 - outColor.b / centerBrightness) : (outColor.b - centerBrightness) / centerBrightness;"+
"outColor.g = outColor.g * hslColor.b * colorize;"+

//\tconvert back to rgb
"outColor = hsv2rgb(outColor);" +

//Additive Blend
"gl_FragColor = vec4(outColor, 1.0); " +
"}" }, 
Em = { uniforms: { tDiffuse: { type: "t", value: null }, time: { type: "f", value: 0 }, strength: { type: "f", value: .001 }, size: { type: "f", value: 50 }, speed: { type: "f", value: 1 } }, 
vertexShader: `" + "\t\t${cm}" + "\t`, 
fragmentShader: "" + 
"uniform sampler2D tDiffuse;" +
"uniform float time;" +
"uniform float strength;" +
"uniform float size;" +
"uniform float speed;"+
"varying vec2 vUv;"+
"const float TWO_PI = 6.283185307179586;"+
"void main() {"+
"vec2 p = -1.0 + 2.0 * vUv;"+
"float pos = time * TWO_PI + length(p * size);"+
"gl_FragColor = texture2D(tDiffuse, vUv + strength * vec2(cos(pos), sin(pos)));" +
"}" }, 
Om = { uniforms: { tDiffuse: { value: null }, offset: { value: 1 }, darkness: { value: 1 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: [
"uniform float offset;", 
"uniform float darkness;", 
"uniform sampler2D tDiffuse;", 
"varying vec2 vUv;", 
"void main() {", 
"vec4 texel = texture2D( tDiffuse, vUv );", 
"vec2 uv = ( vUv - vec2( 0.5 ) ) * vec2( offset );", 
"gl_FragColor = vec4( mix( texel.rgb, vec3( 1.0 - darkness ), dot( uv, uv ) ), texel.a );", 
"}"].join("" + "") }, 
Dm = { uniforms: { tDiffuse: { value: null }, amount: { value: .005 }, angle: { value: 0 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: [
"uniform sampler2D tDiffuse;", 
"uniform float amount;", 
"uniform float angle;", 
"varying vec2 vUv;", 
"void main() {", 
    "vec2 offset = amount * vec2( cos(angle), sin(angle));", 
    "vec4 cr = texture2D(tDiffuse, vUv + offset);", 
    "vec4 cga = texture2D(tDiffuse, vUv);", 
    "vec4 cb = texture2D(tDiffuse, vUv - offset);", 
    "gl_FragColor = vec4(cr.r, cga.g, cb.b, cga.a);", 
"}"].join("" + "") },
Nm = { uniforms: { tDiffuse: { value: null }, side: { value: 1 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: [
    "uniform sampler2D tDiffuse;", 
    "uniform int side;", 
    "varying vec2 vUv;", 
    "void main() {", 
    "vec2 p = vUv;", 
    "if (side == 0){", 
    "if (p.x > 0.5) p.x = 1.0 - p.x;", 
    "}else if (side == 1){", 
    "if (p.x < 0.5) p.x = 1.0 - p.x;", 
    "}else if (side == 2){", 
    "if (p.y < 0.5) p.y = 1.0 - p.y;", 
    "}else if (side == 3){", 
    "if (p.y > 0.5) p.y = 1.0 - p.y;", "} ", 
    "vec4 color = texture2D(tDiffuse, p);", "gl_FragColor = color;", "}"].join("" + "") }, 
Im = { uniforms: { tDiffuse: { value: null }, tSize: { value: [256, 256] }, center: { value: [.5, .5] }, angle: { value: 1.57 }, scale: { value: 1 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: ["uniform vec2 center;", "uniform float angle;", "uniform float scale;", "uniform vec2 tSize;", 
"uniform sampler2D tDiffuse;", 
"varying vec2 vUv;", 
"float pattern() {", 
"float s = sin( angle ), c = cos( angle );", 
"vec2 tex = vUv * tSize - center;", 
"vec2 point = vec2( c * tex.x - s * tex.y, s * tex.x + c * tex.y ) * scale;", 
"return ( sin( point.x ) * sin( point.y ) ) * 4.0;", 
"}", 
"void main() {", 
"vec4 color = texture2D( tDiffuse, vUv );", 
"float average = ( color.r + color.g + color.b ) / 3.0;", 
"gl_FragColor = vec4( vec3( average * 10.0 - 5.0 + pattern() ), color.a );", 
"}"].join("" + "") }, 
zm = { uniforms: { tDiffuse: { value: null }, hue: { value: 0 }, saturation: { value: 0 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: [
"uniform sampler2D tDiffuse;", 
"uniform float hue;", 
"uniform float saturation;", 
"varying vec2 vUv;", 
"void main() {", 
"gl_FragColor = texture2D( tDiffuse, vUv );", 
"float angle = hue * 3.14159265;", 
"float s = sin(angle), c = cos(angle);", 
"vec3 weights = (vec3(2.0 * c, -sqrt(3.0) * s - c, sqrt(3.0) * s - c) + 1.0) / 3.0;", 
"float len = length(gl_FragColor.rgb);", 
"gl_FragColor.rgb = vec3(", 
"dot(gl_FragColor.rgb, weights.xyz),",
    "dot(gl_FragColor.rgb, weights.zxy),", 
    "dot(gl_FragColor.rgb, weights.yzx)", 
    ");", 
"float average = (gl_FragColor.r + gl_FragColor.g + gl_FragColor.b) / 3.0;",
"if (saturation > 0.0) {", 
"gl_FragColor.rgb += (average - gl_FragColor.rgb) * (1.0 - 1.0 / (1.001 - saturation));", 
"} else {", 
"gl_FragColor.rgb += (average - gl_FragColor.rgb) * (-saturation);", 
"}", 
"}"].join("" + "") }, 
Rm = { uniforms: { tDiffuse: { value: null }, brightness: { value: 0 }, contrast: { value: 0 } }, 
vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
fragmentShader: [
"uniform sampler2D tDiffuse;", 
"uniform float brightness;", 
"uniform float contrast;", 
"varying vec2 vUv;", 
"void main() {", 
"gl_FragColor = texture2D( tDiffuse, vUv );", 
"gl_FragColor.rgb += brightness;", 
"if (contrast > 0.0) {", 
"gl_FragColor.rgb = (gl_FragColor.rgb - 0.5) / (1.0 - contrast) + 0.5;", "} else {", 
"gl_FragColor.rgb = (gl_FragColor.rgb - 0.5) * (1.0 + contrast) + 0.5;", 
"}",
"}"].join("" + "") }, 
km = { uniforms: { tDiffuse: { value: null }, v: { value: 1 / 512 }, r: { value: .35 } }, 
vertexShader: [
"varying vec2 vUv;", 
"void main() {", 
    "vUv = uv;", 
    "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", 
"}"].join("" + ""), 

fragmentShader: [
"uniform sampler2D tDiffuse;", 
"uniform float v;", 
"uniform float r;", 
"varying vec2 vUv;",
"void main() {", 
    "vec4 sum = vec4( 0.0 );", 
    "float vv = v * abs( r - vUv.y );", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y - 4.0 * vv ) ) * 0.051;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y - 3.0 * vv ) ) * 0.0918;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y - 2.0 * vv ) ) * 0.12245;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y - 1.0 * vv ) ) * 0.1531;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y ) ) * 0.1633;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y + 1.0 * vv ) ) * 0.1531;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y + 2.0 * vv ) ) * 0.12245;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y + 3.0 * vv ) ) * 0.0918;", 
    "sum += texture2D( tDiffuse, vec2( vUv.x, vUv.y + 4.0 * vv ) ) * 0.051;", 
    "gl_FragColor = sum;", 
"}"].join("" + "") }; 
