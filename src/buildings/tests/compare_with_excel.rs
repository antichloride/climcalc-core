use crate::buildings::tests::buildings_test_case::create_buildings;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;

fn assert(a: [f64; 4], b: [f64; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=1e-6);
    assert_relative_eq!(a[1], b[1], max_relative=1e-6);
    assert_relative_eq!(a[2], b[2], max_relative=1e-6);
    assert_relative_eq!(a[3], b[3], max_relative=1e-6);
}

#[test]
fn test_buildings_calculate() {
    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut buildings = create_buildings(start_year, end_year);

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
	buildings.calculate(year);
	buildings.calculate_second_stage(year, &nrg_own_mix_price__m__eur_per_W_h);
	buildings.calculate_emissions(year);
    }


    // [start:assert_measures]

	// EW bzw. MA bzw. SuS (in 1.000)
	assert(
		buildings.inputs.n_inhabitants__k__.get_year_values(2022),
		[217.0, 116.7053125, 27.999643016826923, 4.059360576923077],
	);
	assert(
		buildings.inputs.n_inhabitants__k__.get_year_values(2023),
		[217.0, 116.7053125, 27.999643016826923, 4.059360576923077],
	);
	assert(
		buildings.inputs.n_inhabitants__k__.get_year_values(2024),
		[217.0, 116.7053125, 27.999643016826923, 4.059360576923077],
	);

	// Anzahl Gebäude
	assert(
		buildings.inputs.n_buildings.get_year_values(2022),
		[44000.0, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);
	assert(
		buildings.inputs.n_buildings.get_year_values(2023),
		[44000.0, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);
	assert(
		buildings.inputs.n_buildings.get_year_values(2024),
		[44000.0, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);

	// qm Geschossfläche/Gebäude
	assert(
		buildings.inputs.floor_A_building__m2.get_year_values(2022),
		[197.68294830471567, 1.9225230799595314, 4.807188524606554, 0.8788371689773329],
	);
	assert(
		buildings.inputs.floor_A_building__m2.get_year_values(2023),
		[197.68294830471567, 1.9225230799595314, 4.807188524606554, 0.8788371689773329],
	);
	assert(
		buildings.inputs.floor_A_building__m2.get_year_values(2024),
		[197.68294830471567, 1.9225230799595314, 4.807188524606554, 0.8788371689773329],
	);

	// Geschossfläche (in 1.000 qm)
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2022),
		[8698.049725407489, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2023),
		[8698.049725407489, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2024),
		[8698.049725407489, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);

	// Heizbedarf (in kWh/qm/a)
	assert(
		buildings.inputs.heat_dmd__k__W_h_per_m2_a.get_year_values(2022),
		[114.43569553805774, 50.0, 50.0, 50.0],
	);
	assert(
		buildings.inputs.heat_dmd__k__W_h_per_m2_a.get_year_values(2023),
		[114.43569553805774, 50.0, 50.0, 50.0],
	);
	assert(
		buildings.inputs.heat_dmd__k__W_h_per_m2_a.get_year_values(2024),
		[111.57855268091488, 50.0, 50.0, 50.0],
	);

	// WW-Bedarf je EW bzw. MA bzw. SuS (in kWh/a)
	assert(
		buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.get_year_values(2022),
		[1237.9807692307693, 10.0, 10.0, 10.0],
	);
	assert(
		buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.get_year_values(2023),
		[1237.9807692307693, 10.0, 10.0, 10.0],
	);
	assert(
		buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.get_year_values(2024),
		[1237.9807692307693, 10.0, 10.0, 10.0],
	);

	// Wärmebedarf insgesamt (in GWh/a)
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2022),
		[1264.009197074695, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2023),
		[1264.009197074695, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2024),
		[1239.1576264306736, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);

	// Strombedarf Gebäude außer WP/ePkw je EW bzw. MA bzw. SuS (in kWh/a)
	assert(
		buildings.inputs.elec_dmd_capita__k_W_h_per_a.get_year_values(2022),
		[1253.2091346153845, 50.0, 50.0, 50.0],
	);
	assert(
		buildings.inputs.elec_dmd_capita__k_W_h_per_a.get_year_values(2023),
		[1253.2091346153845, 50.0, 50.0, 50.0],
	);
	assert(
		buildings.inputs.elec_dmd_capita__k_W_h_per_a.get_year_values(2024),
		[1253.2091346153845, 50.0, 50.0, 50.0],
	);

	// Strombedarf Gebäude außer WP/ePkw (in GWh)
	assert(
		buildings.results.elec_dmd__G__W_h_per_a.get_year_values(2022),
		[271.94638221153843, 5.835265625, 1.3999821508413461, 0.20296802884615386],
	);
	assert(
		buildings.results.elec_dmd__G__W_h_per_a.get_year_values(2023),
		[271.94638221153843, 5.835265625, 1.3999821508413461, 0.20296802884615386],
	);
	assert(
		buildings.results.elec_dmd__G__W_h_per_a.get_year_values(2024),
		[271.94638221153843, 5.835265625, 1.3999821508413461, 0.20296802884615386],
	);

	// Fläche mit Ölheizung ohne Brennwert/Niedertemperatur (in 1.000 qm)
	assert(
		buildings.inputs.A_heat_oil__k__m2.get_year_values(2022),
		[1168.5182735522906, 330.4891601843849, 23.507365709620334, 9.752139152981853],
	);
	assert(
		buildings.inputs.A_heat_oil__k__m2.get_year_values(2023),
		[1168.5182735522906, 330.4891601843849, 23.507365709620334, 9.752139152981853],
	);
	assert(
		buildings.inputs.A_heat_oil__k__m2.get_year_values(2024),
		[1160.1849402189573, 330.4891601843849, 23.507365709620334, 9.752139152981853],
	);

	// Fläche mit Ölheizung mit Brennwert/Niedertemperatur (in 1.000 qm)
	assert(
		buildings.inputs.A_heat_oil_condensing__k__m2.get_year_values(2022),
		[1295.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846],
	);
	assert(
		buildings.inputs.A_heat_oil_condensing__k__m2.get_year_values(2023),
		[1295.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846],
	);
	assert(
		buildings.inputs.A_heat_oil_condensing__k__m2.get_year_values(2024),
		[1285.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846],
	);

	// Fläche mit Gasheizung (in 1.000 qm)
	assert(
		buildings.inputs.A_heat_gas__k__m2.get_year_values(2022),
		[4918.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079],
	);
	assert(
		buildings.inputs.A_heat_gas__k__m2.get_year_values(2023),
		[4918.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079],
	);
	assert(
		buildings.inputs.A_heat_gas__k__m2.get_year_values(2024),
		[4908.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079],
	);

	// Fläche mit Wärmepumpen-Heizung (in 1.000 qm)
	assert(
		buildings.inputs.A_heat_heat_pump__k__m2.get_year_values(2022),
		[278.2399038461538, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.inputs.A_heat_heat_pump__k__m2.get_year_values(2023),
		[278.2399038461538, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.inputs.A_heat_heat_pump__k__m2.get_year_values(2024),
		[268.2399038461538, 0.0, 0.0, 0.0],
	);

	// Heizöl Verbrauch o. Brennwert/Niedertemperatur (in GWh)
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2022),
		[195.18417509599794, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2023),
		[195.18417509599794, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2024),
		[189.982080423786, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);

	// Heizöl Verbrauch o. Brennwert/Niedertemperatur (in GWh)
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2022),
		[192.1636249445945, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2023),
		[192.1636249445945, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2024),
		[186.93180054711846, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);

	// Heizöl Verbrauch (in Mio. l)
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2022),
		[42.34723435713605, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2023),
		[42.34723435713605, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2024),
		[41.20653440721723, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);

	// Gas Verbrauch (in GWh)
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2022),
		[772.7752374268276, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2023),
		[772.7752374268276, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2024),
		[756.0416235908847, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);

	// Gas Verbrauch (in Mio. m3)
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2022),
		[70.25229431152978, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2023),
		[70.25229431152978, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2024),
		[68.73105669008042, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);

	// Strom WP Verbrauch (in GWh)
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2022),
		[13.478032760508308, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2023),
		[13.478032760508308, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2024),
		[12.738162885451509, 0.0, 0.0, 0.0],
	);

	// Andere Wärmequelle (in GWh)
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2022),
		[150.62741939413374, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2023),
		[150.62741939413374, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2024),
		[153.12706144788083, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);

	// Heizöl Kosten (in Mio €)
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2022),
		[34.53840434168016, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2023),
		[34.53840434168016, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2024),
		[33.60804946252637, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);

	// Gas Kosten (in Mio €)
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2022),
		[51.621385860112085, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2023),
		[51.621385860112085, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2024),
		[50.503580455871095, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);

	// Strom WP Kosten (in Mio €)
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2022),
		[3.124339412502536, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2023),
		[3.124339412502536, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2024),
		[2.963414989496691, 0.0, 0.0, 0.0],
	);

	// Invest Heizung (in Mio. €)
	assert(
		buildings.results.invest_heat_sources__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_heat_sources__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_heat_sources__M__eur_per_a.get_year_values(2024),
		[2.1933508311460894, 0.0, 0.0, 0.0],
	);

	// Invest Wärmebedarf (in Mio. €)
	assert(
		buildings.results.invest_energetic_renovation__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_energetic_renovation__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_energetic_renovation__M__eur_per_a.get_year_values(2024),
		[62.12892661005358, 0.0, 0.0, 0.0],
	);

	// Zuschuss Heizung (in Mio. €)
	assert(
		buildings.results.grant_heat_sources__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.grant_heat_sources__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.grant_heat_sources__M__eur_per_a.get_year_values(2024),
		[0.0, 0.0, 0.0, 0.0],
	);

	// Zuschuss Wärmebedarf  (in Mio. €)
	assert(
		buildings.results.grant_energetic_renovation__M__eur_per_a.get_year_values(2022),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.grant_energetic_renovation__M__eur_per_a.get_year_values(2023),
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.grant_energetic_renovation__M__eur_per_a.get_year_values(2024),
		[18.63867798301607, 0.0, 0.0, 0.0],
	);

	// CO2-Emissionen Heizöl (in 1.000 to )
	assert(
		buildings.results.ems_oil__k__to_coe_per_a.get_year_values(2022),
		[118.57225619998094, 38.58131781875213, 0.0, 8.589890423126528],
	);
	assert(
		buildings.results.ems_oil__k__to_coe_per_a.get_year_values(2023),
		[118.57225619998094, 38.58131781875213, 0.0, 8.589890423126528],
	);
	assert(
		buildings.results.ems_oil__k__to_coe_per_a.get_year_values(2024),
		[115.37829634020822, 38.58131781875213, 0.0, 8.589890423126528],
	);

	// CO2-Emissionen Heizgas (in 1.000 to )
	assert(
		buildings.results.ems_gas__k__to_coe_per_a.get_year_values(2022),
		[140.50458862305956, 65.72749232749233, 0.0, 14.633817318848385],
	);
	assert(
		buildings.results.ems_gas__k__to_coe_per_a.get_year_values(2023),
		[140.50458862305956, 65.72749232749233, 0.0, 14.633817318848385],
	);
	assert(
		buildings.results.ems_gas__k__to_coe_per_a.get_year_values(2024),
		[137.46211338016084, 65.72749232749233, 0.0, 14.633817318848385],
	);

    // [end:assert_measures]

}
