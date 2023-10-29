use crate::energy::tests::energy_test_case::create_energy;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;

fn assert(a: [f64; 4], b: [f64; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=1e-6);
    assert_relative_eq!(a[1], b[1], max_relative=1e-6);
    assert_relative_eq!(a[2], b[2], max_relative=1e-6);
    assert_relative_eq!(a[3], b[3], max_relative=1e-6);
}

#[test]
fn test_energy_calculate() {
    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut energy = create_energy(start_year, end_year);

    let mut buildings_elec_dmd__G__W_h_per_a = SectorsResult::new(
	    "buildings_elec_dmd__G__W_h_per_a".to_owned(),
	    start_year,
	    end_year
    );

    let mut cnsmp_elec_heat_pump__G__W_h_per_a = SectorsResult::new(
	    "cnsmp_elec_heat_pump__G__W_h_per_a".to_owned(),
	    start_year,
	    end_year
    );

    let mut bev_elec_nrg_dmd__G__W_h_per_a = SectorsResult::new(
	    "bev_elec_nrg_dmd__G__W_h_per_a".to_owned(),
	    start_year,
	    end_year
    );

    let mut sl_nrg_dmd__G__W_h_per_a = Results::new(
	    "sl_nrg_dmd__G__W_h_per_a".to_owned(),
	    start_year,
	    end_year
    );

    let mut raw_vals: SectorsRawValues;

    // [start:declare_variables]
	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(271.94638221153843,5.835265625,1.3999821508413461,0.20296802884615386);
	buildings_elec_dmd__G__W_h_per_a.set_year_values(
		2045,
		&raw_vals,
		);



	raw_vals=SectorsRawValues::new();
	raw_vals.set(13.478032760508308,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(13.478032760508308,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(12.738162885451509,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(12.01734062944233,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.771397863874567,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.5254550983068,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.279512332739033,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.03356956717127,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.787626801603501,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2045,
		&raw_vals,
		);



	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.0021476583283425656,0.0012768289795447468,0.0,3.8016619808284724e-06);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.0021476583283425656,0.0012768289795447468,0.0,3.8016619808284724e-06);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(29.843523645641127,174.53485895602574,0.0,97.13282183144241);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(59.684899632953915,349.06844108307195,0.0,194.2656398612228);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2045,
		&raw_vals,
		);



	sl_nrg_dmd__G__W_h_per_a.set_year_value(2022,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2023,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2024,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2025,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2026,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2027,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2028,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2029,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2030,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2031,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2032,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2033,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2034,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2035,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2036,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2037,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2038,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2039,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2040,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2041,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2042,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2043,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2044,10.432692307692307);
	sl_nrg_dmd__G__W_h_per_a.set_year_value(2045,10.432692307692307);
    // [end:declare_variables]

    for year in start_year..end_year+1{
	energy.calculate(year);
	energy.calculate_second_stage(
	    year,
	    buildings_elec_dmd__G__W_h_per_a.get_year(year),
	    cnsmp_elec_heat_pump__G__W_h_per_a.get_year(year),
	    bev_elec_nrg_dmd__G__W_h_per_a.get_year(year),
	    sl_nrg_dmd__G__W_h_per_a.get_year(year),
	);
	energy.calculate_emissions(year);
    }

    // [start:assert_measures]

	// Strombedarf (in GWh/a)
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2022),
		[285.4265626303751, 5.836542453979544, 0.0, 12.035646289041788],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2023),
		[285.4265626303751, 5.836542453979544, 0.0, 12.035646289041788],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2024),
		[314.5280687426311, 180.37012458102575, 0.0, 109.16846431882222],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2025),
		[343.64862247393467, 354.9037067080719, 0.0, 206.30128234860263],
	);

	//   MWp Potenzial PV auf Dach
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2022),
		[254.7663461538462, 355.0140865384616, 0.0, 18.684951923076927],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2023),
		[254.7663461538462, 355.0140865384616, 0.0, 18.684951923076927],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2024),
		[254.7663461538462, 355.0140865384616, 0.0, 18.684951923076927],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2025),
		[254.7663461538462, 355.0140865384616, 0.0, 18.684951923076927],
	);

	// PV lokal auf Dach (MWp)
	assert(
		energy.inputs.sol_rf_installed__M__Wp.get_year_values(2022),
		[36.26212389624195, 50.53086871683937, 0.0, 2.6595194061494407],
	);
	assert(
		energy.inputs.sol_rf_installed__M__Wp.get_year_values(2023),
		[36.26212389624195, 50.53086871683937, 0.0, 2.6595194061494407],
	);
	assert(
		energy.inputs.sol_rf_installed__M__Wp.get_year_values(2024),
		[34.59545722957529, 50.53086871683937, 0.0, 2.6595194061494407],
	);
	assert(
		energy.inputs.sol_rf_installed__M__Wp.get_year_values(2025),
		[32.92879056290862, 50.53086871683937, 0.0, 2.6595194061494407],
	);

	// PV lokal auf Dach (in GWh/a)
	assert(
		energy.results.sol_rf_nrg__G__W_h_per_a.get_year_values(2022),
		[30.052235179010516, 41.87745744908062, 0.0, 2.2040767078463492],
	);
	assert(
		energy.results.sol_rf_nrg__G__W_h_per_a.get_year_values(2023),
		[30.052235179010516, 41.87745744908062, 0.0, 2.2040767078463492],
	);
	assert(
		energy.results.sol_rf_nrg__G__W_h_per_a.get_year_values(2024),
		[28.670985179010522, 41.87745744908062, 0.0, 2.2040767078463492],
	);
	assert(
		energy.results.sol_rf_nrg__G__W_h_per_a.get_year_values(2025),
		[27.289735179010517, 41.87745744908062, 0.0, 2.2040767078463492],
	);

	//  % Eigenverbrauch PV auf Dach lokal
	assert(
		energy.inputs.sol_rf_self_cnsmp_part.get_year_values(2022),
		[0.0006520432692307692, 0.0006520432692307692, 0.0, 0.0006520432692307692],
	);
	assert(
		energy.inputs.sol_rf_self_cnsmp_part.get_year_values(2023),
		[0.0006520432692307692, 0.0006520432692307692, 0.0, 0.0006520432692307692],
	);
	assert(
		energy.inputs.sol_rf_self_cnsmp_part.get_year_values(2024),
		[0.0006520432692307692, 0.0006520432692307692, 0.0, 0.0006520432692307692],
	);
	assert(
		energy.inputs.sol_rf_self_cnsmp_part.get_year_values(2025),
		[0.0006520432692307692, 0.0006520432692307692, 0.0, 0.0006520432692307692],
	);

	// PV lokal auf Dach - Eigenverbrauch (in GWh/a)
	assert(
		energy.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year_values(2022),
		[0.01959535767381395, 0.02730591426217096, 0.0, 0.0014371533822195245],
	);
	assert(
		energy.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year_values(2023),
		[0.01959535767381395, 0.02730591426217096, 0.0, 0.0014371533822195245],
	);
	assert(
		energy.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year_values(2024),
		[0.01869472290818895, 0.02730591426217096, 0.0, 0.0014371533822195245],
	);
	assert(
		energy.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year_values(2025),
		[0.01779408814256395, 0.02730591426217096, 0.0, 0.0014371533822195245],
	);

	// Invest gesamt (in Mio. €)
	assert(
		energy.results.sol_rf_invest__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_invest__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_invest__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_invest__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0, 0.0, 0.0],
	);

	// Zuschuss Gesamt (in Mio. €)
	assert(
		energy.results.sol_rf_grant__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_grant__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_grant__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_grant__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0, 0.0, 0.0],
	);

	// O&M
	assert(
		energy.results.sol_rf_om__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_om__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_om__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_om__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0, 0.0, 0.0],
	);

	// Einspeisevergütung (in Mio. €/a)
	assert(
		energy.results.sol_rf_revenue__M__eur_per_a.get_year_values(2022),
		[2.417627505617604, 3.3689371985528846, 0.0, 0.17731248413436243],
	);
	assert(
		energy.results.sol_rf_revenue__M__eur_per_a.get_year_values(2023),
		[2.417627505617604, 3.3689371985528846, 0.0, 0.17731248413436243],
	);
	assert(
		energy.results.sol_rf_revenue__M__eur_per_a.get_year_values(2024),
		[2.3065093817162374, 3.3689371985528846, 0.0, 0.17731248413436243],
	);
	assert(
		energy.results.sol_rf_revenue__M__eur_per_a.get_year_values(2025),
		[2.1953912578148698, 3.3689371985528846, 0.0, 0.17731248413436243],
	);

	// Freifläche belegt mit PV (in Hektar)
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2022),
		[0.0, 10.0, 0.0, 4.0],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2023),
		[0.0, 10.0, 0.0, 4.0],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2024),
		[0.0, 12.941176470588236, 0.0, 6.117647058823529],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2025),
		[0.0, 15.882352941176471, 0.0, 8.235294117647058],
	);

	// Freifläche MWp installiert
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2022),
		[0.0, 0.08333333333333334, 0.0, 0.03333333333333334],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2023),
		[0.0, 0.08333333333333334, 0.0, 0.03333333333333334],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2024),
		[0.0, 0.10784313725490198, 0.0, 0.05098039215686275],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2025),
		[0.0, 0.13235294117647062, 0.0, 0.06862745098039216],
	);

	// Freifläche GWh/a produziert
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2022),
		[0.0, 0.09208333333333334, 0.0, 0.03683333333333334],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2023),
		[0.0, 0.09208333333333334, 0.0, 0.03683333333333334],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2024),
		[0.0, 0.11916666666666668, 0.0, 0.05633333333333334],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2025),
		[0.0, 0.14625000000000002, 0.0, 0.07583333333333334],
	);

	// Invest Gesamt (in Mio. €)
	assert(
		energy.results.sol_os_invest__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_invest__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_invest__M__eur_per_a.get_year_values(2024),
		[0.0, 0.016299019607843142, 0.0, 0.011735294117647057],
	);
	assert(
		energy.results.sol_os_invest__M__eur_per_a.get_year_values(2025),
		[0.0, 0.016299019607843142, 0.0, 0.011735294117647057],
	);

	// Zuschuss gesamt (in Mio. €)
	assert(
		energy.results.sol_os_grant__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_grant__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_grant__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_grant__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0, 0.0, 0.0],
	);

	// O&M
	assert(
		energy.results.sol_os_om__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_om__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_om__M__eur_per_a.get_year_values(2024),
		[0.0, 0.00032598039215686283, 0.0, 0.00023470588235294116],
	);
	assert(
		energy.results.sol_os_om__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0006519607843137257, 0.0, 0.0004694117647058823],
	);

	// Kosten Erzeugung (in Mio. €/a)
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0029555555555555555, 0.0, 0.0011822222222222225],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0029555555555555555, 0.0, 0.0011822222222222225],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2024),
		[0.0, 0.00382483660130719, 0.0, 0.001808104575163399],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2025),
		[0.0, 0.004694117647058824, 0.0, 0.0024339869281045754],
	);

	// Einspeisevergütung (in Mio. €/a)
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2022),
		[0.0, 0.005202708333333333, 0.0, 0.0020810833333333337],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2023),
		[0.0, 0.005202708333333333, 0.0, 0.0020810833333333337],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2024),
		[0.0, 0.006732916666666667, 0.0, 0.0031828333333333335],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2025),
		[0.0, 0.008263125, 0.0, 0.004284583333333333],
	);

	// Gewinn GHD bzw. Kommune
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2022),
		[0.0, 0.002247152777777777, 0.0, 0.0008988611111111111],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2023),
		[0.0, 0.002247152777777777, 0.0, 0.0008988611111111111],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0029080800653594773, 0.0, 0.0013747287581699346],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0035690073529411753, 0.0, 0.0018505964052287576],
	);

	// Menge (GWh)
	assert(
		energy.inputs.prchsd_renewable_nrg__G__W_h_per_a.get_year_values(2022),
		[97.02403846153847, 89.94284855769232, 0.0, 4.7338341346153845],
	);
	assert(
		energy.inputs.prchsd_renewable_nrg__G__W_h_per_a.get_year_values(2023),
		[97.02403846153847, 89.94284855769232, 0.0, 4.7338341346153845],
	);
	assert(
		energy.inputs.prchsd_renewable_nrg__G__W_h_per_a.get_year_values(2024),
		[97.02403846153847, 89.94284855769232, 0.0, 4.7338341346153845],
	);
	assert(
		energy.inputs.prchsd_renewable_nrg__G__W_h_per_a.get_year_values(2025),
		[97.02403846153847, 89.94284855769232, 0.0, 4.7338341346153845],
	);

	// Preis (€/kWh)
	assert(
		energy.inputs.renewable_nrg_price__m__eur_per_W_h.get_year_values(2022),
		[0.3263, 0.2323, 0.0, 0.2323],
	);
	assert(
		energy.inputs.renewable_nrg_price__m__eur_per_W_h.get_year_values(2023),
		[0.3263, 0.2323, 0.0, 0.2323],
	);
	assert(
		energy.inputs.renewable_nrg_price__m__eur_per_W_h.get_year_values(2024),
		[0.3263, 0.2323, 0.0, 0.2323],
	);
	assert(
		energy.inputs.renewable_nrg_price__m__eur_per_W_h.get_year_values(2025),
		[0.3263, 0.2323, 0.0, 0.2323],
	);

	// Kosten (in Mio. €/a)
	assert(
		energy.results.prchsd_renewable_nrg__M__eur_per_a.get_year_values(2022),
		[31.65894375, 20.893723719951925, 0.0, 1.0996696694711539],
	);
	assert(
		energy.results.prchsd_renewable_nrg__M__eur_per_a.get_year_values(2023),
		[31.65894375, 20.893723719951925, 0.0, 1.0996696694711539],
	);
	assert(
		energy.results.prchsd_renewable_nrg__M__eur_per_a.get_year_values(2024),
		[31.65894375, 20.893723719951925, 0.0, 1.0996696694711539],
	);
	assert(
		energy.results.prchsd_renewable_nrg__M__eur_per_a.get_year_values(2025),
		[31.65894375, 20.893723719951925, 0.0, 1.0996696694711539],
	);

	// Menge (GWh)
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2022),
		[188.3829288111628, -84.13361201797494, 0.0, 7.300375001044183],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2023),
		[188.3829288111628, -84.13361201797494, 0.0, 7.300375001044183],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2024),
		[217.48533555818443, 90.39997010907126, 0.0, 104.43319303082463],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2025),
		[246.60678992425363, 264.9335522361174, 0.0, 201.566011060605],
	);

	// Kosten (in Mio. €/a)
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2022),
		[61.46934967108242, 0.0, 0.0, 1.6958771127425638],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2023),
		[61.46934967108242, 0.0, 0.0, 1.6958771127425638],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2024),
		[70.96546499263557, 20.999913056337256, 0.0, 24.25983074106056],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2025),
		[80.46779555228396, 61.54406418445008, 0.0, 46.823784369378544],
	);

	// CO2-Emissionen Strom (in 1.000 to )
	assert(
		energy.results.prchsd_nrg_mix_ems__k__to_coe_per_a.get_year_values(2022),
		[63.526140344323125, -28.371379925228148, 0.0, 2.4618188591143726],
	);
	assert(
		energy.results.prchsd_nrg_mix_ems__k__to_coe_per_a.get_year_values(2023),
		[63.526140344323125, -28.371379925228148, 0.0, 2.4618188591143726],
	);
	assert(
		energy.results.prchsd_nrg_mix_ems__k__to_coe_per_a.get_year_values(2024),
		[73.33999973718825, 30.48450952807983, 0.0, 35.216766821436295],
	);
	assert(
		energy.results.prchsd_nrg_mix_ems__k__to_coe_per_a.get_year_values(2025),
		[83.16028233266775, 89.34039898138778, 0.0, 67.97171478375819],
	);

    // [end:assert_measures]
}
