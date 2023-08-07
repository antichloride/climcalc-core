use crate::energy::tests::energy_test_case::create_energy;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;

fn assert(a: [f32; 4], b: [f32; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=0.3);
    assert_relative_eq!(a[1], b[1], max_relative=0.3);
    assert_relative_eq!(a[2], b[2], max_relative=0.3);
    assert_relative_eq!(a[3], b[3], max_relative=0.3);
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
	raw_vals.set(13.120849358974361,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(13.120849358974361,0.0,0.0,0.0);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(12.544091185575786,0.523954236021925,1.4508939458723298,0.5937805086192182);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.97517614943211,1.4718627080657753,4.252681837616991,1.681341525857655);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.873905598904209,2.453104513442959,7.087803062694982,2.8022358764294246);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.772635048376303,3.434346318820143,9.922924287772977,3.9231302270011947);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.671364497848401,4.415588124197327,12.75804551285097,5.044024577572965);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.5700939473205,5.396829929574509,15.593166737928962,6.164918928144734);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.468823396792594,6.378071734951693,18.428287963006955,7.285813278716504);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.367552846264692,7.359313540328877,21.26340918808495,8.406707629288276);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.266282295736787,8.34055534570606,24.098530413162944,9.527601979860043);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.165011745208885,9.321797151083244,26.933651638240935,10.648496330431813);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(11.063741194680981,10.303038956460428,29.76877286331893,11.769390681003584);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.962470644153079,11.284280761837612,32.60389408839692,12.890285031575354);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.861200093625174,12.265522567214795,35.43901531347492,14.011179382147125);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.75992954309727,13.246764372591977,38.27413653855291,15.132073732718894);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.658658992569366,14.228006177969162,41.1092577636309,16.252968083290664);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.557388442041463,15.209247983346346,43.9443789887089,17.373862433862435);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,16.19048978872353,46.77950021378689,18.494756784434205);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,17.17173159410071,49.61462143886488,19.615651135005972);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,18.152973399477897,52.44974266394288,20.736545485577746);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,19.134215204855078,55.284863889020876,21.857439836149513);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,20.115457010232262,58.11998511409886,22.978334186721284);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(10.456117891513559,21.096698815609447,60.95510633917686,24.099228537293055);
	cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(
		2045,
		&raw_vals,
		);



	raw_vals=SectorsRawValues::new();
	raw_vals.set(3.667043534201671e-06,4.647866049660458e-07,0.0,1.5975639810911312e-09);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(3.667043534201671e-06,4.647866049660458e-07,0.0,1.5975639810911312e-09);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.05095666241613036,0.0635335396063165,0.0,0.040817910251410135);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.10190965778872652,0.12706661442602804,0.0,0.08163581890525629);
	bev_elec_nrg_dmd__G__W_h_per_a.set_year_values(
		2045,
		&raw_vals,
		);



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
    }

    // [start:assert_measures]

	// Strombedarf (in GWh/a)
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2022),
		[285.0672352375563, 5.835266089786605, 0.0, 1.6029501812850642],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2023),
		[285.0672352375563, 5.835266089786605, 0.0, 1.6029501812850642],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2024),
		[284.54143005953034, 6.422753400628242, 0.0, 3.6884425444304583],
	);
	assert(
		energy.results.elec_nrg_dmd__G__W_h_per_a.get_year_values(2025),
		[284.0234680187593, 7.434194947491804, 0.0, 7.618609362067402],
	);

	//   MWp Potenzial PV auf Dach
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2022),
		[1.019765742382495, 1.421032286958801, 0.0, 0.07479117299783164],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2023),
		[1.019765742382495, 1.421032286958801, 0.0, 0.07479117299783164],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2024),
		[1.019765742382495, 1.421032286958801, 0.0, 0.07479117299783164],
	);
	assert(
		energy.results.sol_rf_potential__M__Wp.get_year_values(2025),
		[1.019765742382495, 1.421032286958801, 0.0, 0.07479117299783164],
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
		[26.262123896241953, 60.530868716839365, 0.0, -7.340480593850559],
	);
	assert(
		energy.inputs.sol_rf_installed__M__Wp.get_year_values(2025),
		[16.262123896241953, 70.53086871683936, 0.0, -17.340480593850558],
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
		[21.76473517901052, 50.164957449080624, 0.0, -6.083423292153651],
	);
	assert(
		energy.results.sol_rf_nrg__G__W_h_per_a.get_year_values(2025),
		[13.477235179010519, 58.452457449080626, 0.0, -14.37092329215365],
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
		[0.01419154908006395, 0.03270972285592096, 0.0, -0.003966655211530476],
	);
	assert(
		energy.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year_values(2025),
		[0.00878774048631395, 0.03811353144967096, 0.0, -0.009370463805280475],
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
		[0.0, 12.999999999999991, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_invest__M__eur_per_a.get_year_values(2025),
		[0.0, 13.0, 0.0, 0.0],
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
		[0.0, 0.25999999999999984, 0.0, 0.0],
	);
	assert(
		energy.results.sol_rf_om__M__eur_per_a.get_year_values(2025),
		[0.0, 0.5199999999999999, 0.0, 0.0],
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
		[1.7509187622094013, 4.035645941961088, 0.0, -0.4893962592738407],
	);
	assert(
		energy.results.sol_rf_revenue__M__eur_per_a.get_year_values(2025),
		[1.0842100188011983, 4.702354685369292, 0.0, -1.1561050026820436],
	);

	// Freifläche belegt mit PV (in Hektar)
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2022),
		[0.0, 78.2451923076923, 0.0, 0.0],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2023),
		[0.0, 78.2451923076923, 0.0, 0.0],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2024),
		[0.0, 88.2451923076923, 0.0, 10.0],
	);
	assert(
		energy.inputs.sol_os_installed_A__ha.get_year_values(2025),
		[0.0, 98.2451923076923, 0.0, 20.0],
	);

	// Freifläche MWp installiert
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2022),
		[0.0, 65.20432692307692, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2023),
		[0.0, 65.20432692307692, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2024),
		[0.0, 73.53766025641026, 0.0, 8.333333333333336],
	);
	assert(
		energy.results.sol_os_installed__M__Wp.get_year_values(2025),
		[0.0, 81.87099358974359, 0.0, 16.66666666666667],
	);

	// Freifläche GWh/a produziert
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2022),
		[0.0, 72.05078125, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2023),
		[0.0, 72.05078125, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2024),
		[0.0, 81.25911458333334, 0.0, 9.208333333333336],
	);
	assert(
		energy.results.sol_os_nrg__G__W_h_per_a.get_year_values(2025),
		[0.0, 90.46744791666667, 0.0, 18.41666666666667],
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
		[0.0, 5.541666666666673, 0.0, 5.541666666666668],
	);
	assert(
		energy.results.sol_os_invest__M__eur_per_a.get_year_values(2025),
		[0.0, 5.541666666666663, 0.0, 5.541666666666668],
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
		[0.0, 0.11083333333333346, 0.0, 0.11083333333333335],
	);
	assert(
		energy.results.sol_os_om__M__eur_per_a.get_year_values(2025),
		[0.0, 0.2216666666666667, 0.0, 0.2216666666666667],
	);

	// Kosten Erzeugung (in Mio. €/a)
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2022),
		[0.0, 6.027777777777778, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2023),
		[0.0, 6.027777777777778, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2024),
		[0.0, 6.798148148148148, 0.0, 0.7703703703703705],
	);
	assert(
		energy.results.sol_os_prod_costs__M__eur_per_a.get_year_values(2025),
		[0.0, 7.568518518518518, 0.0, 1.540740740740741],
	);

	// Einspeisevergütung (in Mio. €/a)
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2022),
		[0.0, 4.070869140625, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2023),
		[0.0, 4.070869140625, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2024),
		[0.0, 4.591139973958334, 0.0, 0.5202708333333335],
	);
	assert(
		energy.results.sol_os_turnover_buyback__M__eur_per_a.get_year_values(2025),
		[0.0, 5.111410807291667, 0.0, 1.040541666666667],
	);

	// Gewinn GHD bzw. Kommune
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2022),
		[0.0, -1.956908637152778, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2023),
		[0.0, -1.956908637152778, 0.0, 0.0],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2024),
		[0.0, -2.2070081741898147, 0.0, -0.250099537037037],
	);
	assert(
		energy.results.sol_os_revenue__M__eur_per_a.get_year_values(2025),
		[0.0, -2.4571077112268513, 0.0, -0.500199074074074],
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
		[188.02360141834401, -84.13488838216789, 0.0, -3.13232110671254],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2023),
		[188.02360141834401, -84.13488838216789, 0.0, -3.13232110671254],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2024),
		[187.50320004891182, -83.55280487991999, 0.0, -1.0414249349733957],
	);
	assert(
		energy.results.prchsd_nrg_mix__G__W_h_per_a.get_year_values(2025),
		[186.99064181673452, -82.54676714165018, 0.0, 2.8941456912572976],
	);

	// Kosten (in Mio. €/a)
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2022),
		[61.35210114280565, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2023),
		[61.35210114280565, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2024),
		[61.182294175959925, 0.0, 0.0, 0.0],
	);
	assert(
		energy.results.prchsd_nrg_mix_costs__M__eur_per_a.get_year_values(2025),
		[61.015046424800474, 0.0, 0.0, 0.6723100440790702],
	);

    // [end:assert_measures]
}
