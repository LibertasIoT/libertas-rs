use libertas_macros::{
    LibertasExport, libertas_chart, libertas_chart_channel, libertas_chart_guide,
    libertas_chart_scale,
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
#[derive(LibertasExport)]
struct Sample {
    #[libertas_chart_channel(x, tooltip)]
    #[libertas_chart_scale(id = time, kind = utc)]
    #[libertas_chart_guide(target = x, source = scale)]
    at: i64,
}

#[test]
fn chart_helpers_compile_on_aliases_and_derived_fields() {
    let _: Option<History> = None;
    let _: Timestamp = 0;
}
