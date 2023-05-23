use crate::buildings::tests::buildings_test_case::create_buildings;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;

fn assert(a: [f32; 4], b: [f32; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=0.1);
    assert_relative_eq!(a[1], b[1], max_relative=0.1);
    assert_relative_eq!(a[2], b[2], max_relative=0.1);
    assert_relative_eq!(a[3], b[3], max_relative=0.1);
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
	raw_vals.set(0.31780241830274375,0.31780241830274375,0.31780241830274375,0.31780241830274375);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2022,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31780241830274375,0.31780241830274375,0.31780241830274375,0.31780241830274375);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2023,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177991644825993,0.3177991644825993,0.3177991644825993,0.3177991644825993);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31779590816965125,0.31779590816965125,0.31779590816965125,0.31779590816965125);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31779264936103385,0.31779264936103385,0.31779264936103385,0.31779264936103385);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31778938805387696,0.31778938805387696,0.31778938805387696,0.31778938805387696);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31778612424530595,0.31778612424530595,0.31778612424530595,0.31778612424530595);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177828579324419,0.3177828579324419,0.3177828579324419,0.3177828579324419);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31777958911240145,0.31777958911240145,0.31777958911240145,0.31777958911240145);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177763177822967,0.3177763177822967,0.3177763177822967,0.3177763177822967);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177730439392355,0.3177730439392355,0.3177730439392355,0.3177730439392355);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31776976758032105,0.31776976758032105,0.31776976758032105,0.31776976758032105);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177664887026522,0.3177664887026522,0.3177664887026522,0.3177664887026522);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177632073033234,0.3177632073033234,0.3177632073033234,0.3177632073033234);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31775992337942455,0.31775992337942455,0.31775992337942455,0.31775992337942455);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.317756636928041,0.317756636928041,0.317756636928041,0.317756636928041);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177533479462538,0.3177533479462538,0.3177533479462538,0.3177533479462538);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3177500564311393,0.3177500564311393,0.3177500564311393,0.3177500564311393);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.31774676237976957,0.31774676237976957,0.31774676237976957,0.31774676237976957);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2045,
		&raw_vals,
		);

    // [end:declare_variables]

    for year in start_year..end_year+1{
	buildings.calculate(year);
	buildings.calculate_second_stage(year, &nrg_own_mix_price__m__eur_per_W_h);
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
	assert(
		buildings.inputs.n_inhabitants__k__.get_year_values(2025),
		[217.0, 116.7053125, 27.999643016826923, 4.059360576923077],
	);

	// Anzahl Gebäude
	assert(
		buildings.inputs.n_buildings.get_year_values(2022),
		[50268.065649038464, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);
	assert(
		buildings.inputs.n_buildings.get_year_values(2023),
		[50268.065649038464, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);
	assert(
		buildings.inputs.n_buildings.get_year_values(2024),
		[50268.065649038464, 5180.149927884616, 147.35656250000002, 334.386045673077],
	);
	assert(
		buildings.inputs.n_buildings.get_year_values(2025),
		[50268.065649038464, 5180.149927884616, 147.35656250000002, 334.386045673077],
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
	assert(
		buildings.inputs.floor_A_building__m2.get_year_values(2025),
		[197.68294830471567, 1.9225230799595314, 4.807188524606554, 0.8788371689773329],
	);

	// Geschossfläche (in 1.000 qm)
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2022),
		[9937.139423076924, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2023),
		[9937.139423076924, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2024),
		[9937.139423076924, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
	);
	assert(
		buildings.results.floor_A__k__m2.get_year_values(2025),
		[9937.139423076924, 9.958957794008876, 0.7083707762754685, 0.2938708857248521],
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
		[113.25922494982245, 50.0, 50.0, 50.0],
	);
	assert(
		buildings.inputs.heat_dmd__k__W_h_per_m2_a.get_year_values(2025),
		[112.08275436158715, 50.0, 50.0, 50.0],
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
	assert(
		buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.get_year_values(2025),
		[1237.9807692307693, 10.0, 10.0, 10.0],
	);

	// Wärmebedarf insgesamt (in GWh/a)
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2022),
		[1405.8052884615388, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2023),
		[1405.8052884615388, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2024),
		[1394.1145361990953, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2025),
		[1382.4237839366517, 1.665001014700444, 0.31541496898204263, 0.055287150055473376],
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
	assert(
		buildings.inputs.elec_dmd_capita__k_W_h_per_a.get_year_values(2025),
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
	assert(
		buildings.results.elec_dmd__G__W_h_per_a.get_year_values(2025),
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
		[1158.5182735522906, 330.4891601843849, 23.507365709620334, 9.752139152981853],
	);
	assert(
		buildings.inputs.A_heat_oil__k__m2.get_year_values(2025),
		[1148.5182735522906, 330.4891601843849, 23.507365709620334, 9.752139152981853],
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
		[1295.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846],
	);
	assert(
		buildings.inputs.A_heat_oil_condensing__k__m2.get_year_values(2025),
		[1295.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846],
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
		[4918.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079],
	);
	assert(
		buildings.inputs.A_heat_gas__k__m2.get_year_values(2025),
		[4918.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079],
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
		[278.2399038461538, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.inputs.A_heat_heat_pump__k__m2.get_year_values(2025),
		[278.2399038461538, 0.0, 0.0, 0.0],
	);

	// Fläche mit anderer Wärmequelle (in 1.000 qm)
	assert(
		buildings.inputs.A_heat_other__k__m2.get_year_values(2022),
		[2275.604927884616, -2687.14040118035, -191.13362776679344, -79.29266757581362],
	);
	assert(
		buildings.inputs.A_heat_other__k__m2.get_year_values(2023),
		[2275.604927884616, -2687.14040118035, -191.13362776679344, -79.29266757581362],
	);
	assert(
		buildings.inputs.A_heat_other__k__m2.get_year_values(2024),
		[2285.604927884616, -2687.14040118035, -191.13362776679344, -79.29266757581362],
	);
	assert(
		buildings.inputs.A_heat_other__k__m2.get_year_values(2025),
		[2295.604927884616, -2687.14040118035, -191.13362776679344, -79.29266757581362],
	);

	// Heizöl Verbrauch o. Brennwert/Niedertemperatur (in GWh)
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2022),
		[190.0115695069493, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2023),
		[190.0115695069493, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2024),
		[186.8188559006308, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2025),
		[183.6531875951913, 63.50948302669535, 12.031128775338722, 2.108862569476673],
	);

	// Heizöl Verbrauch o. Brennwert/Niedertemperatur (in GWh)
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2022),
		[187.0710674157304, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2023),
		[187.0710674157304, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2024),
		[185.51537437446893, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2025),
		[183.95968133320747, 62.526649359583324, 11.844942431973386, 2.076227110426836],
	);

	// Heizöl Verbrauch (in Mio. l)
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2022),
		[41.22498384164861, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2023),
		[41.22498384164861, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2024),
		[40.70585893863644, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2025),
		[40.18969079359336, 13.779042078125762, 2.61027836698119, 0.4575396412782846],
	);

	// Gas Verbrauch (in GWh)
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2022),
		[752.2958030145531, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2023),
		[752.2958030145531, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2024),
		[746.0396707227591, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2025),
		[739.7835384309649, 361.5012078012078, 68.48217582984668, 12.003819423819428],
	);

	// Gas Verbrauch (in Mio. m3)
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2022),
		[68.39052754677756, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2023),
		[68.39052754677756, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2024),
		[67.82178824752356, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2025),
		[67.25304894826954, 32.863746163746164, 6.2256523481678805, 1.0912563112563116],
	);

	// Strom WP Verbrauch (in GWh)
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2022),
		[13.120849358974361, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2023),
		[13.120849358974361, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2024),
		[13.011735671191554, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2025),
		[12.902621983408748, 0.0, 0.0, 0.0],
	);

	// Andere Wärmequelle (in GWh)
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2022),
		[321.92941105769245, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2023),
		[321.92941105769245, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2024),
		[320.65516224644017, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2025),
		[319.35738402342315, -449.2529828070334, -85.10572329150475, -14.9176588206405],
	);

	// Heizöl Kosten (in Mio €)
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2022),
		[33.6230968212486, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2023),
		[33.6230968212486, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2024),
		[33.199698550351876, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2025),
		[32.778711811254745, 11.238186718919371, 2.1289430361098587, 0.37316933142656894],
	);

	// Gas Kosten (in Mio €)
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2022),
		[50.25335964137216, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2023),
		[50.25335964137216, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2024),
		[49.83545000428031, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2025),
		[49.41754036718846, 24.14828068112068, 4.574609345433759, 0.8018551375111378],
	);

	// Strom WP Kosten (in Mio €)
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2022),
		[3.0414384377548194, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2023),
		[3.0414384377548194, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2024),
		[3.0161147915905606, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2025),
		[2.9907916398863845, 0.0, 0.0, 0.0],
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
		[0.5721784776902886, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_heat_sources__M__eur_per_a.get_year_values(2025),
		[0.5662961247491123, 0.0, 0.0, 0.0],
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
		[29.226880656108495, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.invest_energetic_renovation__M__eur_per_a.get_year_values(2025),
		[29.22688065610885, 0.0, 0.0, 0.0],
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
	assert(
		buildings.results.grant_heat_sources__M__eur_per_a.get_year_values(2025),
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
		[8.76806419683255, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.grant_energetic_renovation__M__eur_per_a.get_year_values(2025),
		[8.768064196832652, 0.0, 0.0, 0.0],
	);

    // [end:assert_measures]

}
