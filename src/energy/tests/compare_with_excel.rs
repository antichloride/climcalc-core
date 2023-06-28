use crate::energy::tests::energy_test_case::create_energy;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;

fn assert(a: [f32; 4], b: [f32; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=0.1);
    assert_relative_eq!(a[1], b[1], max_relative=0.1);
    assert_relative_eq!(a[2], b[2], max_relative=0.1);
    assert_relative_eq!(a[3], b[3], max_relative=0.1);
}

#[test]
fn test_energy_calculate() {
    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut energy = create_energy(start_year, end_year);

    // let mut nrg_own_mix_price__m__eur_per_W_h = SectorsResult::new(
	    // "nrg_own_mix_price__m__eur_per_W_h".to_owned(),
	    // start_year,
	    // end_year
    // );
}
