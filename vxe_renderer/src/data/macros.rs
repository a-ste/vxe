/// Macro for easier creation of vertices
#[macro_export]
macro_rules! vertex {
    ($x:expr, $y:expr, $z:expr) => {
        {
            Vertex::new(
                VertexPosition::new([$x, $y, $z]),
                VertexNormal::new([0.0, 0.0, 0.0]),
                VertexRGB::new([1.0, 1.0, 1.0]),
            )
        }
    };

    ($x:expr, $y:expr, $z:expr, $nx:expr, $ny:expr, $nz:expr) => {
        {
            Vertex::new(
                VertexPosition::new([$x, $y, $z]),
                VertexNormal::new([$nx, $ny, $nz]),
                VertexRGB::new([1.0, 1.0, 1.0]),
            )
        }
    };

    ($x:expr, $y:expr, $z:expr, $nx:expr, $ny:expr, $nz:expr, $r:expr, $g:expr, $b:expr) => {
        {
            Vertex::new(
                VertexPosition::new([$x, $y, $z]),
                VertexNormal::new([$nx, $ny, $nz]),
                VertexRGB::new([$r, $g, $b]),
            )
        }
    };
}

/// Macro for creation of shader interfaces, import of data::shader::* is required for it to work
#[macro_export]
macro_rules! shd_interface {
    ($sn:ident, $($n:ident, $t:ty),*) => {
        #[derive(Debug, UniformInterface)]
        pub struct $sn {
            $(
                #[uniform(unbound)]
                $n: Uniform<$t>,
            )*
        }
    }
}