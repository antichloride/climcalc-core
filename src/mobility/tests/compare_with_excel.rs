use crate::mobility::tests::mobility_test_case::create_mobility;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;

fn assert(a: [f64; 4], b: [f64; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=1e-6);
    assert_relative_eq!(a[1], b[1], max_relative=1e-6);
    assert_relative_eq!(a[2], b[2], max_relative=1e-6);
    assert_relative_eq!(a[3], b[3], max_relative=1e-6);
}

#[test]
fn test_mobility_calculate() {
    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut mobility = create_mobility(start_year, end_year);

    let mut nrg_own_mix_price__m__eur_per_W_h = SectorsResult::new(
	    "nrg_own_mix_price__m__eur_per_W_h".to_owned(),
	    start_year,
	    end_year
    );

    let mut raw_vals: SectorsRawValues;

    // [start:declare_variables]
	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3178131160242466,0.3178131160242466,0.3178131160242466,0.3178131160242466);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3178131160242466,0.3178131160242466,0.3178131160242466,0.3178131160242466);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31895233950000823,0.31895233950000823,0.31895233950000823,0.31895233950000823);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3198989593172235,0.3198989593172235,0.3198989593172235,0.3198989593172235);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3202185909643858,0.3202185909643858,0.3202185909643858,0.3202185909643858);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32053868077589576,0.32053868077589576,0.32053868077589576,0.32053868077589576);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3208592297375685,0.3208592297375685,0.3208592297375685,0.3208592297375685);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3211802388380492,0.3211802388380492,0.3211802388380492,0.3211802388380492);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32150170906882347,0.32150170906882347,0.32150170906882347,0.32150170906882347);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32182685657380034,0.32182685657380034,0.32182685657380034,0.32182685657380034);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32215200407877714,0.32215200407877714,0.32215200407877714,0.32215200407877714);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32247715158375395,0.32247715158375395,0.32247715158375395,0.32247715158375395);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32280229908873087,0.32280229908873087,0.32280229908873087,0.32280229908873087);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3231274465937077,0.3231274465937077,0.3231274465937077,0.3231274465937077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2045,
		&raw_vals,
		);



    // [end:declare_variables]


    for year in start_year..end_year+1{
        mobility.calculate(year);
        mobility.calculate_second_stage(
            year,
            nrg_own_mix_price__m__eur_per_W_h.get_year(year),
        );
	mobility.calculate_emissions(year);
    }

    // [start:assert_measures]

	// Anzahl Pkw (in 1.000)
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2022),
		[87.0, 8.0, 0.0, 0.5],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2023),
		[87.0, 8.0, 0.0, 0.5],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2024),
		[87.0, 8.0, 0.0, 0.5],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2025),
		[87.0, 8.0, 0.0, 0.5],
	);

	//   davon Anzahl Pkw BEV (in 1.000)
	assert(
	mobility.inputs.n_bev__k__.get_year_values(2022),
		[0.000719691454326923, 7.315663633233172e-05, 0.0, 3.9138800437797477e-07],
	);
	assert(
	mobility.inputs.n_bev__k__.get_year_values(2023),
		[0.000719691454326923, 7.315663633233172e-05, 0.0, 3.9138800437797477e-07],
	);
	assert(
	mobility.inputs.n_bev__k__.get_year_values(2024),
		[10.000719691454327, 10.000073156636333, 0.0, 10.000000391388005],
	);
	assert(
	mobility.inputs.n_bev__k__.get_year_values(2025),
		[20.00071969145433, 20.000073156636333, 0.0, 20.000000391388003],
	);

	// Fahrleistung/Pkw (in 1.000 km)
	assert(
	mobility.results.traveld_dist_car__M__m_per_a.get_year_values(2022),
		[19.067971876877177, 111.52305567223398, 0.0, 62.06569842158493],
	);
	assert(
	mobility.results.traveld_dist_car__M__m_per_a.get_year_values(2023),
		[19.067971876877177, 111.52305567223398, 0.0, 62.06569842158493],
	);
	assert(
	mobility.results.traveld_dist_car__M__m_per_a.get_year_values(2024),
		[19.067971876877177, 111.52305567223398, 0.0, 62.06569842158493],
	);
	assert(
	mobility.results.traveld_dist_car__M__m_per_a.get_year_values(2025),
		[19.067971876877177, 111.52305567223398, 0.0, 62.06569842158493],
	);

	// Zuschuss in Mio. €)
	assert(
	mobility.results.cars_grant__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
	mobility.results.cars_grant__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
	mobility.results.cars_grant__M__eur_per_a.get_year_values(2024),
		[90.0, 90.0, 0.0, 90.0],
	);
	assert(
	mobility.results.cars_grant__M__eur_per_a.get_year_values(2025),
		[90.00000000000001, 90.0, 0.0, 89.99999999999999],
	);

	// Verbrauch Strom (in GWh/a)
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2022),
		[0.0021476583283425656, 0.0012768289795447468, 0.0, 3.8016619808284724e-06],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2023),
		[0.0021476583283425656, 0.0012768289795447468, 0.0, 3.8016619808284724e-06],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2024),
		[29.843523645641127, 174.53485895602574, 0.0, 97.13282183144241],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2025),
		[59.684899632953915, 349.06844108307195, 0.0, 194.2656398612228],
	);

	// Verbrauch Benzin/Diesel (in Mio. l/a)
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2022),
		[94.55729032321851, 50.854048343395995, 0.0, 1.7688710203842892],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2023),
		[94.55729032321851, 50.854048343395995, 0.0, 1.7688710203842892],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2024),
		[83.68854635339851, -12.71409338977739, 0.0, -33.60857707991913],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2025),
		[72.81980238357852, -76.28223512295077, 0.0, -68.98602518022254],
	);

	// Kosten Strom (in Mio. €/a)
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2022),
		[0.0006825539854859753, 0.000405792996619175, 0.0, 1.2082180401980066e-06],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2023),
		[0.0006825539854859753, 0.000405792996619175, 0.0, 1.2082180401980066e-06],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2024),
		[9.518661685701053, 55.668301588328376, 0.0, 30.980740765376034],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2025),
		[19.093137279534893, 111.66663103296027, 0.0, 62.14537602269971],
	);

	// Kosten Benzin/Diesel (in Mio. €/a)
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2022),
		[176.58611790777184, 94.97013869748538, 0.0, 3.303373706051741],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2023),
		[176.58611790777184, 94.97013869748538, 0.0, 3.303373706051741],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2024),
		[156.28869506915711, -23.74362026178283, 0.0, -62.76415213105729],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2025),
		[135.9912722305424, -142.45737922105104, 0.0, -128.8316779681663],
	);

	// CO2-Emissionen Benzin (in 1.000 to )
	assert(
	mobility.results.cars_ems__k__to_coe_per_a.get_year_values(2022),
		[264.76041290501183, 142.39133536150877, 0.0, 4.952838857076009],
	);
	assert(
	mobility.results.cars_ems__k__to_coe_per_a.get_year_values(2023),
		[264.76041290501183, 142.39133536150877, 0.0, 4.952838857076009],
	);
	assert(
	mobility.results.cars_ems__k__to_coe_per_a.get_year_values(2024),
		[234.3279297895158, -35.59946149137669, 0.0, -94.10401582377355],
	);
	assert(
	mobility.results.cars_ems__k__to_coe_per_a.get_year_values(2025),
		[203.89544667401984, -213.59025834426214, 0.0, -193.1608705046231],
	);

	// Straßenbeleuchtung
	assert_relative_eq!(
		mobility.inputs.n_sl__k__.get_year(2022),
		24.12560096153846,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.n_sl__k__.get_year(2023),
		24.12560096153846,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.n_sl__k__.get_year(2024),
		24.12560096153846,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.n_sl__k__.get_year(2025),
		24.12560096153846,
		max_relative=0.1,
	);

	// nan
	assert_relative_eq!(
		mobility.inputs.nrg_cnsmp_per_sl__k__W_h_per_a.get_year(2022),
		432.43243243243245,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.nrg_cnsmp_per_sl__k__W_h_per_a.get_year(2023),
		432.43243243243245,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.nrg_cnsmp_per_sl__k__W_h_per_a.get_year(2024),
		432.43243243243245,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.nrg_cnsmp_per_sl__k__W_h_per_a.get_year(2025),
		432.43243243243245,
		max_relative=0.1,
	);

	// nan
	assert_relative_eq!(
		mobility.inputs.om_costs_per_sl__eur_per_a.get_year(2022),
		119.54032432432432,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.om_costs_per_sl__eur_per_a.get_year(2023),
		119.54032432432432,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.om_costs_per_sl__eur_per_a.get_year(2024),
		119.54032432432432,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.inputs.om_costs_per_sl__eur_per_a.get_year(2025),
		119.54032432432432,
		max_relative=0.1,
	);

	// nan
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2022),
		3.31564645082988,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2023),
		3.31564645082988,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2024),
		3.327531618822201,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2025),
		3.337407412107572,
		max_relative=0.1,
	);

	// nan
	assert_relative_eq!(
		mobility.results.sl_om_costs__M__eur_per_a.get_year(2022),
		2.883982163461538,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_om_costs__M__eur_per_a.get_year(2023),
		2.883982163461538,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_om_costs__M__eur_per_a.get_year(2024),
		2.883982163461538,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_om_costs__M__eur_per_a.get_year(2025),
		2.883982163461538,
		max_relative=0.1,
	);

	// nan
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2022),
		6.199628614291418,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2023),
		6.199628614291418,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2024),
		6.211513782283739,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2025),
		6.22138957556911,
		max_relative=0.1,
	);

    // [end:assert_measures]
}
