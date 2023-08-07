use crate::buildings::tests::buildings_test_case::create_buildings;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;

fn assert(a: [f32; 4], b: [f32; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=0.1);
    assert_relative_eq!(a[1], b[1], max_relative=6.0);
    assert_relative_eq!(a[2], b[2], max_relative=6.0);
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
	raw_vals.set(0.32013442264468817,0.32013442264468817,0.32013442264468817,0.32013442264468817);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2024,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32247516720508346,0.32247516720508346,0.32247516720508346,0.32247516720508346);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2025,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32247380294781064,0.32247380294781064,0.32247380294781064,0.32247380294781064);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2026,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32247243771697265,0.32247243771697265,0.32247243771697265,0.32247243771697265);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2027,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3224710715115269,0.3224710715115269,0.3224710715115269,0.3224710715115269);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2028,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32246970433042954,0.32246970433042954,0.32246970433042954,0.32246970433042954);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2029,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.322468336172635,0.322468336172635,0.322468336172635,0.322468336172635);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2030,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32246696703709615,0.32246696703709615,0.32246696703709615,0.32246696703709615);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2031,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32246559692276494,0.32246559692276494,0.32246559692276494,0.32246559692276494);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2032,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32246422582859097,0.32246422582859097,0.32246422582859097,0.32246422582859097);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2033,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3224628537535228,0.3224628537535228,0.3224628537535228,0.3224628537535228);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2034,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3224614806965077,0.3224614806965077,0.3224614806965077,0.3224614806965077);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2035,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3224601066564909,0.3224601066564909,0.3224601066564909,0.3224601066564909);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2036,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245873163241656,0.32245873163241656,0.32245873163241656,0.32245873163241656);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2037,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.3224573556232271,0.3224573556232271,0.3224573556232271,0.3224573556232271);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2038,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245597862786346,0.32245597862786346,0.32245597862786346,0.32245597862786346);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2039,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2040,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2041,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2042,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2043,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
	nrg_own_mix_price__m__eur_per_W_h.set_year_values(
		2044,
		&raw_vals,
		);

	raw_vals=SectorsRawValues::new();
	raw_vals.set(0.32245460064526515,0.32245460064526515,0.32245460064526515,0.32245460064526515);
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
		[113.25922494982245, 40.0, 40.0, 40.0],
	);
	assert(
		buildings.inputs.heat_dmd__k__W_h_per_m2_a.get_year_values(2025),
		[112.08275436158715, 30.0, 30.0, 30.0],
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
		[1394.1145361990953, 1.565411436760355, 0.30833126121928794, 0.052348441198224856],
	);
	assert(
		buildings.results.total_heat_dmd__G__W_h_per_a.get_year_values(2025),
		[1382.4237839366517, 1.4658218588202665, 0.3012475534565333, 0.049409732340976335],
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
		[1158.5182735522906, 320.4891601843849, 23.507365709620334, 9.752139152981853],
	);
	assert(
		buildings.inputs.A_heat_oil__k__m2.get_year_values(2025),
		[1148.5182735522906, 300.4891601843849, 23.507365709620334, 9.752139152981853],
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
		[1285.8923033707865, 356.51404494382024, 26.069779981285112, 10.815168539325846],
	);
	assert(
		buildings.inputs.A_heat_oil_condensing__k__m2.get_year_values(2025),
		[1275.8923033707865, 336.51404494382024, 26.069779981285112, 10.815168539325846],
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
		[4908.884014423077, 1990.0961538461538, 132.26485285216347, 59.01923076923079],
	);
	assert(
		buildings.inputs.A_heat_gas__k__m2.get_year_values(2025),
		[4898.884014423077, 1970.0961538461538, 112.26485285216347, 59.01923076923079],
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
	assert(
		buildings.inputs.A_heat_heat_pump__k__m2.get_year_values(2025),
		[258.2399038461538, 0.0, 0.0, 0.0],
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
		[186.8188559006308, 57.9040183026613, 11.760929169481019, 1.996769015994123],
	);
	assert(
		buildings.results.cnsmp_oil__G__W_h_per_a.get_year_values(2025),
		[183.6531875951913, 50.83664241993096, 11.490729563623313, 1.8846754625115731],
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
		[184.08380962258389, 57.18276858662008, 11.578924268899048, 1.965868247780654],
	);
	assert(
		buildings.results.cnsmp_oil_condensing__G__W_h_per_a.get_year_values(2025),
		[181.120561433279, 50.541068723793806, 11.312906105824712, 1.8555093851344717],
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
		[40.54935151032746, 12.581992553719001, 2.5516557556612183, 0.43321978042422177],
	);
	assert(
		buildings.results.cnsmp_oil__M__L_per_a.get_year_values(2025),
		[39.879300813947985, 11.083232412688183, 2.4930331443412466, 0.4088999195701589],
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
		[744.5229859045457, 338.1792356427999, 62.238575434021136, 11.36577368577369],
	);
	assert(
		buildings.results.cnsmp_gas__G__W_h_per_a.get_year_values(2025),
		[736.775605996446, 313.48227677296956, 51.61369737590698, 10.727727947727953],
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
		[67.68390780950416, 30.74356687661817, 5.65805231218374, 1.0332521532521535],
	);
	assert(
		buildings.results.cnsmp_gas__M__m3_per_a.get_year_values(2025),
		[66.97960054513145, 28.498388797542688, 4.692154306900634, 0.9752479952479957],
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
		[12.544091185575786, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year_values(2025),
		[11.97517614943211, 0.0, 0.0, 0.0],
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
		[324.86396261698206, -417.66599067103255, -78.84170517621982, -14.124732144882366],
	);
	assert(
		buildings.results.cnsmp_other__G__W_h_per_a.get_year_values(2025),
		[327.7043965292129, -382.2634104108344, -68.52500522331792, -13.331805469124228],
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
		[33.07205109182308, 10.261873126813217, 2.0811304343172896, 0.3533340529139953],
	);
	assert(
		buildings.results.costs_oil__M__eur_per_a.get_year_values(2025),
		[32.52555774385598, 9.039484355788481, 2.0333178325247205, 0.3334987744014216],
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
		[49.73413545842366, 22.590372940939034, 4.157536838992612, 0.7592336822096823],
	);
	assert(
		buildings.results.costs_gas__M__eur_per_a.get_year_values(2025),
		[49.216610480562586, 20.940616088434368, 3.447794984710586, 0.7166122269082272],
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
		[2.9290815282028713, 0.0, 0.0, 0.0],
	);
	assert(
		buildings.results.costs_heat_pump__M__eur.get_year_values(2025),
		[2.8166836334705194, 0.0, 0.0, 0.0],
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
		[2.2887139107611545, 0.75, 0.25, 0.0],
	);
	assert(
		buildings.results.invest_heat_sources__M__eur_per_a.get_year_values(2025),
		[2.265184498996449, 1.2, 0.4, 0.0],
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
		[29.226880656108495, 0.2489739448502219, 0.01770926940688671, 0.007346772143121302],
	);
	assert(
		buildings.results.invest_energetic_renovation__M__eur_per_a.get_year_values(2025),
		[29.22688065610885, 0.2489739448502219, 0.01770926940688671, 0.007346772143121302],
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
		[8.76806419683255, 0.07469218345506656, 0.005312780822066014, 0.0022040316429363907],
	);
	assert(
		buildings.results.grant_energetic_renovation__M__eur_per_a.get_year_values(2025),
		[8.768064196832652, 0.07469218345506656, 0.005312780822066014, 0.0022040316429363907],
	);

    // [end:assert_measures]

}
