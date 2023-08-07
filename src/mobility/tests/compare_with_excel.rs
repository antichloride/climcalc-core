use crate::mobility::tests::mobility_test_case::create_mobility;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;

fn assert(a: [f32; 4], b: [f32; 4]){
    assert_relative_eq!(a[0], b[0], max_relative=0.1);
    assert_relative_eq!(a[1], b[1], max_relative=0.1);
    assert_relative_eq!(a[2], b[2], max_relative=0.1);
    assert_relative_eq!(a[3], b[3], max_relative=0.1);
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
        mobility.calculate(year);
        mobility.calculate_second_stage(
            year,
            nrg_own_mix_price__m__eur_per_W_h.get_year(year),
        );
    }

    // [start:assert_measures]

	// Anzahl Pkw (in 1.000)
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2022),
		[112.37370452706975, 12.791540597281763, 0.0, 0.6732393680983818],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2023),
		[112.37370452706975, 12.791540597281763, 0.0, 0.6732393680983818],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2024),
		[112.37370452706975, 12.791540597281763, 0.0, 0.6732393680983818],
	);
	assert(
	mobility.inputs.n_cars__k__.get_year_values(2025),
		[112.37370452706975, 12.791540597281763, 0.0, 0.6732393680983818],
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
		[0.000719691454326923, 7.315663633233172e-05, 0.0, 3.9138800437797477e-07],
	);
	assert(
	mobility.inputs.n_bev__k__.get_year_values(2025),
		[0.000719691454326923, 7.315663633233172e-05, 0.0, 3.9138800437797477e-07],
	);

	// Fahrleistung/Pkw (in 1.000 km)
	assert(
	mobility.inputs.traveld_dist_car__M__m_per_a.get_year_values(2022),
		[0.03255782451923077, 0.04059621394230769, 0.0, 0.02608173076923077],
	);
	assert(
	mobility.inputs.traveld_dist_car__M__m_per_a.get_year_values(2023),
		[0.03255782451923077, 0.04059621394230769, 0.0, 0.02608173076923077],
	);
	assert(
	mobility.inputs.traveld_dist_car__M__m_per_a.get_year_values(2024),
		[0.03255782451923077, 0.04059621394230769, 0.0, 0.02608173076923077],
	);
	assert(
	mobility.inputs.traveld_dist_car__M__m_per_a.get_year_values(2025),
		[0.03255782451923077, 0.04059621394230769, 0.0, 0.02608173076923077],
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
		[0.0, 0.0, 0.0, 0.0],
	);
	assert(
	mobility.results.cars_grant__M__eur_per_a.get_year_values(2025),
		[0.0, 0.0, 0.0, 0.0],
	);

	// Verbrauch Strom (in GWh/a)
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2022),
		[3.667043534201671e-06, 4.647866049660458e-07, 0.0, 1.5975639810911312e-09],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2023),
		[3.667043534201671e-06, 4.647866049660458e-07, 0.0, 1.5975639810911312e-09],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2024),
		[3.667043534201671e-06, 4.647866049660458e-07, 0.0, 1.5975639810911312e-09],
	);
	assert(
	mobility.results.bev_elec_nrg_dmd__G__W_h_per_a.get_year_values(2025),
		[3.667043534201671e-06, 4.647866049660458e-07, 0.0, 1.5975639810911312e-09],
	);

	// Verbrauch Benzin/Diesel (in Mio. l/a)
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2022),
		[0.20854133549586837, 0.02959925348482078, 0.0, 0.0010008765508330122],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2023),
		[0.20854133549586837, 0.02959925348482078, 0.0, 0.0010008765508330122],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2024),
		[0.20854133549586837, 0.02959925348482078, 0.0, 0.0010008765508330122],
	);
	assert(
	mobility.results.cars_fuel_dmd__M__L_per_a.get_year_values(2025),
		[0.20854133549586837, 0.02959925348482078, 0.0, 0.0010008765508330122],
	);

	// Kosten Strom (in Mio. €/a)
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2022),
		[1.1653953031907312e-06, 1.477103070529314e-07, 0.0, 5.077096965841203e-10],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2023),
		[1.1653953031907312e-06, 1.477103070529314e-07, 0.0, 5.077096965841203e-10],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2024),
		[1.1653833712906092e-06, 1.477087947209133e-07, 0.0, 5.077044983982566e-10],
	);
	assert(
	mobility.results.bev_elec_nrg_price__G__W_h_per_a.get_year_values(2025),
		[1.1653714302492676e-06, 1.4770728123027348e-07, 0.0, 5.076992962299796e-10],
	);

	// Kosten Benzin/Diesel (in Mio. €/a)
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2022),
		[0.3894517782038761, 0.05527672427991674, 0.0, 0.0018691409621868535],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2023),
		[0.3894517782038761, 0.05527672427991674, 0.0, 0.0018691409621868535],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2024),
		[0.3894517782038761, 0.05527672427991674, 0.0, 0.0018691409621868535],
	);
	assert(
	mobility.results.cars_fuel_costs__M__eur_per_a.get_year_values(2025),
		[0.3894517782038761, 0.05527672427991674, 0.0, 0.0018691409621868535],
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
		3.3155348447930475,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2023),
		3.3155348447930475,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2024),
		3.315500898688656,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_nrg_costs__M__eur_per_a.get_year(2025),
		3.3154669265776113,
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
		6.199517008254586,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2023),
		6.199517008254586,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2024),
		6.199483062150194,
		max_relative=0.1,
	);
	assert_relative_eq!(
		mobility.results.sl_total_costs__M__eur_per_a.get_year(2025),
		6.199449090039149,
		max_relative=0.1,
	);

    // [end:assert_measures]
}
