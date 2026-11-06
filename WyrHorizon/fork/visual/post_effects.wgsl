
// WYR: adapted equivalents of the original random Java post shaders.
@fragment fn fs_wyr_invert(in: Out) -> @location(0) vec4<f32> {
    let c=read_in(in.uv);return vec4(vec3(1.0)-c.rgb,c.a);
}
@fragment fn fs_wyr_grayscale(in: Out) -> @location(0) vec4<f32> {
    let c=read_in(in.uv);let y=dot(c.rgb,vec3(0.299,0.587,0.114));return vec4(vec3(y),c.a);
}
@fragment fn fs_wyr_scanline(in: Out) -> @location(0) vec4<f32> {
    let c=read_in(in.uv);let line=u32(in.uv.y*f32(textureDimensions(in_tex).y));
    return vec4(c.rgb*select(0.45,1.0,(line&1u)==0u),c.a);
}
@fragment fn fs_wyr_posterize(in: Out) -> @location(0) vec4<f32> {
    let c=read_in(in.uv);return vec4(floor(c.rgb*5.0+0.5)/5.0,c.a);
}
@fragment fn fs_wyr_chromatic(in: Out) -> @location(0) vec4<f32> {
    let step=vec2(4.0/f32(textureDimensions(in_tex).x),0.0);
    let c=read_in(in.uv);return vec4(read_in(clamp(in.uv+step,vec2(0.0),vec2(1.0))).r,c.g,
        read_in(clamp(in.uv-step,vec2(0.0),vec2(1.0))).b,c.a);
}
@fragment fn fs_wyr_edges(in: Out) -> @location(0) vec4<f32> {
    let step=1.0/vec2<f32>(textureDimensions(in_tex));let c=read_in(in.uv);
    let x=read_in(clamp(in.uv+vec2(step.x,0.0),vec2(0.0),vec2(1.0))).rgb;
    let y=read_in(clamp(in.uv+vec2(0.0,step.y),vec2(0.0),vec2(1.0))).rgb;
    let edge=clamp(length(c.rgb-x)*3.0+length(c.rgb-y)*3.0,0.0,1.0);
    return vec4(vec3(edge),c.a);
}
