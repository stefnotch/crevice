minty_impl! {
    mint::Vector2<f32> => ultraviolet::Vec2,
    mint::Vector3<f32> => ultraviolet::Vec3,
    mint::Vector4<f32> => ultraviolet::Vec4,

    mint::Vector2<f64> => ultraviolet::DVec2,
    mint::Vector3<f64> => ultraviolet::DVec3,
    mint::Vector4<f64> => ultraviolet::DVec4,

    mint::ColumnMatrix2<f32> => ultraviolet::Mat2,
    mint::ColumnMatrix3<f32> => ultraviolet::Mat3,
    mint::ColumnMatrix4<f32> => ultraviolet::Mat4,

    mint::ColumnMatrix2<f64> => ultraviolet::DMat2,
    mint::ColumnMatrix3<f64> => ultraviolet::DMat3,
    mint::ColumnMatrix4<f64> => ultraviolet::DMat4,
}
