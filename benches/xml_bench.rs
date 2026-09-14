use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

use socha::{connection::handler::{ConnectionHandler, Joined}, game::parser::Blokus2026};
use xml::EventReader;

fn xml_test_fn(xml: &[u8]) {
    let parser = EventReader::new(xml);
    let a = ConnectionHandler::<Joined, Blokus2026>::parse_message(parser);
}

fn xml_result_benchmark(c: &mut Criterion) {
    c.bench_function("xml_result_benchmark", |b| b.iter(|| xml_test_fn(black_box(br#"
    <room roomId="ee6d6fba-9e3b-404d-b1ad-511ea1b46ea7">
  <data class="result">
    <definition>
      <fragment name="Siegpunkte">
        <aggregation>SUM</aggregation>
        <relevantForRanking>true</relevantForRanking>
      </fragment>
      <fragment name="Punkte">
        <aggregation>AVERAGE</aggregation>
        <relevantForRanking>true</relevantForRanking>
      </fragment>
    </definition>
    <scores>
      <entry>
        <player name="Spieler 1" team="ONE"/>
        <score>
          <part>2</part>
          <part>121</part>
        </score>
      </entry>
      <entry>
        <player name="Spieler 2" team="TWO"/>
        <score>
          <part>0</part>
          <part>106</part>
        </score>
      </entry>
    </scores>
    <winner team="ONE" regular="true" reason="Spieler 1 hat am meisten Punkte erzielt."/>
  </data>
</room>"#
))));
}

criterion_group!(benches, xml_result_benchmark);
criterion_main!(benches);