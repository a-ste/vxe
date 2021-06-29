/// Macro for easier creation of vertices
#[macro_export]
macro_rules! vertex {
    ($x:expr, $y:expr, $z:expr) => {
        {
            Vertex::new(
                VertexPosition::new([$x, $y, $z]),
                VertexRGB::new([255, 255, 255]),
            )
        }
    };

    ($x:expr, $y:expr, $z:expr, $r:expr, $g:expr, $b:expr) => {
        {
            Vertex::new(
                VertexPosition::new([$x, $y, $z]),
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
        struct $sn {
            $(
                #[uniform(unbound)]
                $n: Uniform<$t>,
            )*
        }
    }
}