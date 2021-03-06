extern crate glutin;
#[macro_use]
extern crate glium;

use glium::{Texture, Surface};

mod support;

#[test]
#[should_fail]
fn empty_pixel_buffer() {
    let display = support::build_display();

    let pixel_buffer = glium::pixel_buffer::PixelBuffer::new_empty(&display, 128 * 128);
    display.assert_no_error();

    let _: Vec<Vec<(u8, u8, u8)>> = pixel_buffer.read_if_supported().unwrap();
}

#[test]
fn texture_2d_read_pixelbuffer() {
    let display = support::build_display();

    // we use only powers of two, in order to avoid float rounding errors
    let texture = glium::texture::Texture2d::new(&display, vec![
        vec![(0u8, 1u8, 2u8), (4u8, 8u8, 16u8)],
        vec![(32u8, 64u8, 128u8), (32u8, 16u8, 4u8)],
    ]);

    let read_back: Vec<Vec<(u8, u8, u8)>> = match texture.read_to_pixel_buffer()
                                                         .read_if_supported() {
        Some(d) => d,
        None => return
    };

    assert_eq!(read_back[0][0], (0, 1, 2));
    assert_eq!(read_back[0][1], (4, 8, 16));
    assert_eq!(read_back[1][0], (32, 64, 128));
    assert_eq!(read_back[1][1], (32, 16, 4));

    display.assert_no_error();
}

macro_rules! read_texture_test {
    ($test_name:ident, $tex_ty:ident, $data_ty:ty, $data:expr) => (
        #[test]
        fn $test_name() {
            let display = support::build_display();

            let texture = glium::texture::$tex_ty::new(&display, $data);

            let read_back: Vec<Vec<$data_ty>> = texture.read();

            assert_eq!(read_back, $data);

            display.assert_no_error();
        }
    );

    ($test_name:ident, maybe $tex_ty:ident, $data_ty:ty, $data:expr) => (
        #[test]
        fn $test_name() {
            let display = support::build_display();

            let texture = match glium::texture::$tex_ty::new_if_supported(&display, $data) {
                Some(t) => t,
                None => return
            };

            let read_back: Vec<Vec<$data_ty>> = texture.read();

            assert_eq!(read_back, $data);

            display.assert_no_error();
        }
    );
}

/*read_texture_test!(read_compressedtexture1d, CompressedTexture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_compressedtexture1darray, CompressedTexture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_compressedtexture2d, CompressedTexture2d, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_compressedtexture2darray, CompressedTexture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_compressedtexture3d, CompressedTexture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);*/
/*read_texture_test!(read_depthstenciltexture1d, maybe DepthStencilTexture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_depthstenciltexture1darray, maybe DepthStencilTexture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_depthstenciltexture2d, maybe DepthStencilTexture2d, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_depthstenciltexture2darray, maybe DepthStencilTexture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_depthstenciltexture3d, DepthStencilTexture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);]*/
/*read_texture_test!(read_depthtexture1d, maybe DepthTexture1d, f32,
    vec![2.0, 3.0]);
read_texture_test!(read_depthtexture1darray, maybe DepthTexture1dArray, f32,
    vec![vec![2.0, 3.0], vec![16.0, 18.0]]);
read_texture_test!(read_depthtexture2d, maybe DepthTexture2d, f32,
    vec![vec![2.0, 3.0], vec![16.0, 18.0]]);
read_texture_test!(read_depthtexture2darray, maybe DepthTexture2dArray, f32,
    vec![vec![2.0, 3.0], vec![16.0, 18.0]]);
read_texture_test!(read_depthtexture3d, DepthTexture3d, f32,
    vec![vec![2.0, 3.0], vec![16.0, 18.0]]);
read_texture_test!(read_integraltexture1d, maybe IntegralTexture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_integraltexture1darray, maybe IntegralTexture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_integraltexture2d, maybe IntegralTexture2d, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_integraltexture2darray, maybe IntegralTexture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_integraltexture3d, maybe IntegralTexture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_stenciltexture1d, maybe StencilTexture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_stenciltexture1darray, maybe StencilTexture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_stenciltexture2d, maybe StencilTexture2d, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_stenciltexture2darray, maybe StencilTexture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_stenciltexture3d, maybe StencilTexture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_texture1d, Texture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_texture1darray, Texture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);*/
read_texture_test!(read_texture2d, Texture2d, (u8, u8, u8, u8),
    vec![
        vec![(0u8, 1u8, 2u8, 3u8), (4u8, 5u8, 6u8, 7u8)],
        vec![(8u8, 9u8, 10u8, 11u8), (12u8, 13u8, 14u8, 15u8)]
    ]);
/*read_texture_test!(read_texture2darray, Texture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_texture3d, Texture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_unsignedtexture1d, maybe UnsignedTexture1d, (u8, u8, u8, u8),
    vec![(0, 1, 2, 3), (4, 5, 6, 7)]);
read_texture_test!(read_unsignedtexture1darray, maybe UnsignedTexture1dArray, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_unsignedtexture2d, maybe UnsignedTexture2d, (u8, u8, u8, u8),
    vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]);
read_texture_test!(read_unsignedtexture2darray, maybe UnsignedTexture2dArray, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);
read_texture_test!(read_unsignedtexture3d, maybe UnsignedTexture3d, (u8, u8, u8, u8),
    vec![vec![vec![(0, 1, 2, 3), (4, 5, 6, 7)], vec![(8, 9, 10, 11), (12, 13, 14, 15)]]]);*/
