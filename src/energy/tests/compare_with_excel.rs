use crate::energy::tests::energy_test_case::create_energy;
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
fn test_energy_calculate() {
    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut energy = create_energy(start_year, end_year);

    let mut electric_power_demand_buildings = SectorsResult::new(
	    "electric_power_demand_buildings".to_owned(),
	    start_year,
	    end_year
    );

    let mut energy_heating_heat_pump = SectorsResult::new(
	    "energy_heating_heat_pump".to_owned(),
	    start_year,
	    end_year
    );

    let mut bev_electric_power_demand = SectorsResult::new(
	    "bev_electric_power_demand".to_owned(),
	    start_year,
	    end_year
    );

    let mut sl_nrg_dmd__G__W_h_per_a = Results::new(
	    "sl_nrg_dmd__G__W_h_per_a".to_owned(),
	    start_year,
	    end_year
    );

    let mut nrg_own_mix_price__m__eur_per_W_h = SectorsResult::new(
	    "nrg_own_mix_price__m__eur_per_W_h".to_owned(),
	    start_year,
	    end_year
    );

    // [start:declare_variables]
    // [end:declare_variables]

    for year in start_year..end_year+1{
	energy.calculate(year);
	energy.calculate_second_stage(
	    year,
	    &electric_power_demand_buildings,
	    &energy_heating_heat_pump,
	    &bev_electric_power_demand,
	    &sl_nrg_dmd__G__W_h_per_a,
	);
    }

    // [start:assert_measures]
    // [end:assert_measures]
}
