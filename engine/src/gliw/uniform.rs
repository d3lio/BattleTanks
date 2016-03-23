pub enum UniformData<'a> {
    /// tuple `Float1(v0)`
    Float1(f32),
    /// tuple `Float2(v0, v1)`
    Float2(f32, f32),
    /// tuple `Float3(v0, v1, v2)`
    Float3(f32, f32, f32),
    /// tuple `Float4(v0, v1, v2, v3)`
    Float4(f32, f32, f32, f32),

    /// tuple `Int1(v0)`
    Int1(i32),
    /// tuple `Int2(v0, v1)`
    Int2(i32, i32),
    /// tuple `Int3(v0, v1, v2)`
    Int3(i32, i32, i32),
    /// tuple `Int4(v0, v1, v2, v3)`
    Int4(i32, i32, i32, i32),

    /// tuple `Uint1(v0)`
    Uint1(u32),
    /// tuple `Uint2(v0, v1)`
    Uint2(u32, u32),
    /// tuple `Uint3(v0, v1, v2)`
    Uint3(u32, u32, u32),
    /// tuple `Uint4(v0, v1, v2, v3)`
    Uint4(u32, u32, u32, u32),

    /// tuple `FloatVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4. <br>
    /// `slice` must be a `&[f32]` with lenght multiple of `size`.
    FloatVec(i32, &'a [f32]),

    /// tuple `IntVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4. <br>
    /// `slice` must be a `&[i32]` with lenght multiple of `size`.
    IntVec(i32, &'a [i32]),

    /// tuple `UintVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4. <br>
    /// `slice` must be a `&[u32]` with lenght multiple of `size`.
    UintVec(i32, &'a [u32]),

    /// tuple `FloatMat(size, transpose, slice)` - an NxN matrix. <br>
    /// `size` can be 2, 3 or 4. <br>
    /// `transpose` spceifies whether the matrix should be passed to the shader as is or transposed. <br>
    /// `slice` must be a `&[f32]` with lenght muptiple of `size * size`.
    FloatMat(i32, bool, &'a [f32]),

    /// tuple `FloatMatNxM(n, m, transpose, slice)` - an NxM matrix. <br>
    /// `n` and `m` can be 2, 3 or 4. <br>
    /// `transpose` spceifies whether the matrix should be passed to the shader as is or transposed. <br>
    /// `slice` must be a `&[f32]` with lenght muptiple of `n * m`. <br>
    FloatMatNxM(i32, i32, bool, &'a [f32]),
}
