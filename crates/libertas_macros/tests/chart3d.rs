use libertas_macros::{
    LibertasExport, libertas_chart3d, libertas_chart3d_channel, libertas_chart3d_guide,
    libertas_chart3d_scale, libertas_chart3d_view, libertas_export,
};

#[allow(dead_code)]
#[libertas_chart3d(surface)]
#[libertas_chart3d_view(
    projection = perspective,
    azimuth = 45,
    elevation = 30,
    zoom = 1,
    fov = 45,
    orbit = true,
    zoomEnabled = false
)]
type Surface = Vec<Sample3D>;

#[allow(dead_code)]
#[libertas_chart3d_channel(z, tooltip)]
#[libertas_chart3d_scale(id = height, kind = linear, growMin = 0.1, growMax = 0.2)]
#[libertas_chart3d_guide(target = z, source = scale)]
type Height = f64;

#[allow(dead_code)]
#[derive(LibertasExport)]
struct Sample3D {
    #[libertas_chart3d_channel(x)]
    x: f64,
    #[libertas_chart3d_channel(y)]
    y: f64,
    #[libertas_chart3d_channel(z, tooltip)]
    #[libertas_chart3d_scale(id = height, kind = linear)]
    #[libertas_chart3d_guide(target = z, source = scale)]
    z: Height,
}

#[allow(dead_code)]
#[derive(LibertasExport)]
enum Scene3D {
    #[libertas_chart3d(surface)]
    #[libertas_chart3d_view(projection = perspective)]
    Surface(Vec<Sample3D>),
}

#[allow(dead_code)]
#[libertas_export]
fn chart3d_parameter_helpers_compile(
    #[libertas_chart3d(surface)]
    #[libertas_chart3d_channel(z)]
    #[libertas_chart3d_scale(kind = linear)]
    #[libertas_chart3d_guide(target = z, source = scale)]
    #[libertas_chart3d_view(projection = perspective)]
    _surface: Surface,
) {
}

#[test]
fn chart3d_helpers_compile_on_aliases_variants_fields_and_parameters() {
    let _: Option<Surface> = None;
    let _: Height = 0.0;
}
