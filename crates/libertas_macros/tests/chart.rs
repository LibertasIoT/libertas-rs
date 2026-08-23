use libertas_macros::{
    LibertasExport, libertas_chart, libertas_chart_channel, libertas_chart_guide,
    libertas_chart_scale, libertas_physical_unit,
};

#[allow(dead_code)]
#[libertas_chart(line)]
type History = Vec<Sample>;

#[allow(dead_code)]
#[libertas_chart_channel(x, tooltip)]
#[libertas_chart_scale(id = time, kind = utc)]
#[libertas_chart_guide(target = x, source = scale)]
type Timestamp = i64;

#[allow(dead_code)]
#[libertas_physical_unit("millimeter")]
type Rainfall = f64;

#[allow(dead_code)]
#[derive(LibertasExport)]
struct Sample {
    #[libertas_chart_channel(x, tooltip)]
    #[libertas_chart_scale(id = time, kind = utc)]
    #[libertas_chart_guide(target = x, source = scale)]
    at: i64,
    #[libertas_physical_unit("millimeter")]
    rainfall: f64,
}

#[test]
fn chart_helpers_compile_on_aliases_and_derived_fields() {
    let _: Option<History> = None;
    let _: Timestamp = 0;
    let _: Rainfall = 0.0;
}
