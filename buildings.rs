pub fn buildings(
    mut input_values: json::JsonValue,
    constants: &json::JsonValue,
    prev_years_values: &json::JsonValue,
) -> json::JsonValue {

     input_values["buildings"]["consumption"] = json::JsonValue::new_object();
    input_values["buildings"]["costs"] = json::JsonValue::new_object();
    input_values["buildings"]["invest"] = json::JsonValue::new_object();

    for sector in ["private", "industry", "schools", "public"]{

        input_values["buildings"]["base_data"]["floor_area[k*qm]"][sector] = json::JsonValue::from(
            input_values["buildings"]["base_data"]["floor_area_per_building[qm]"][sector].as_f32().unwrap()
            * input_values["buildings"]["base_data"]["n_buildings[]"][sector].as_f32().unwrap() /1000.0);

        input_values["buildings"]["demand"]["thermal_energy[G*W*h/a]"][sector] = json::JsonValue::from(
            (input_values["buildings"]["base_data"]["inhabitants[k]"][sector].as_f32().unwrap()
            * input_values["buildings"]["demand"]["hot_water_per_capita[k*W*h/a]"][sector].as_f32().unwrap()
            + input_values["buildings"]["demand"]["heat[k*W*h/qm/a]"][sector].as_f32().unwrap()
            * input_values["buildings"]["base_data"]["floor_area[k*qm]"][sector].as_f32().unwrap())
            /1000.0);

        input_values["buildings"]["demand"]["electric_power[G*W*h/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["demand"]["electric_power_per_capita[k*W*h/a]"][sector].as_f32().unwrap()
            * input_values["buildings"]["base_data"]["inhabitants[k]"][sector].as_f32().unwrap()
            / 1000.0);


        // Consumption

        ////in MWh/a/qm
        let thermal_energy_per_floor_area: f32 =
            input_values["buildings"]["demand"]["thermal_energy[G*W*h/a]"][sector].as_f32().unwrap()
            /input_values["buildings"]["base_data"]["floor_area[k*qm]"][sector].as_f32().unwrap();

        input_values["buildings"]["consumption"]["oil_no_condensing[G*W*h/a]"][sector] = json::JsonValue::from(
        input_values["buildings"]["heat_types_areas"]["oil_no_condensing[k*qm]"][sector].as_f32().unwrap()
        * thermal_energy_per_floor_area
        / constants["buildings"]["heat_types"]["oil_no_condensing"]["efficency[]"].as_f32().unwrap());

        input_values["buildings"]["consumption"]["oil_with_condensing[G*W*h/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["heat_types_areas"]["oil_with_condensing[k*qm]"][sector].as_f32().unwrap()
            * thermal_energy_per_floor_area
            / constants["buildings"]["heat_types"]["oil_with_condensing"]["efficency[]"].as_f32().unwrap());

        input_values["buildings"]["consumption"]["gas[G*W*h/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["heat_types_areas"]["gas[k*qm]"][sector].as_f32().unwrap()
            * thermal_energy_per_floor_area
            / constants["buildings"]["heat_types"]["gas"]["efficency[]"].as_f32().unwrap());

        input_values["buildings"]["consumption"]["heat_pump[G*W*h/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["heat_types_areas"]["heat_pump[k*qm]"][sector].as_f32().unwrap()
            * thermal_energy_per_floor_area
            / constants["buildings"]["heat_types"]["heat_pump"]["efficency[]"].as_f32().unwrap());

        input_values["buildings"]["consumption"]["other[G*W*h/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["heat_types_areas"]["other[k*qm]"][sector].as_f32().unwrap()
            * thermal_energy_per_floor_area);

        input_values["buildings"]["consumption"]["oil[M*L/a]"][sector] = json::JsonValue::from(
            (input_values["buildings"]["consumption"]["oil_no_condensing[G*W*h/a]"][sector].as_f32().unwrap()
            + input_values["buildings"]["consumption"]["oil_with_condensing[G*W*h/a]"][sector].as_f32().unwrap())
            / constants["buildings"]["energy_sources"]["oil"]["calories[k*W*h/L]"].as_f32().unwrap());

        input_values["buildings"]["consumption"]["gas[M*m^3/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["consumption"]["gas[G*W*h/a]"][sector].as_f32().unwrap()
            / constants["buildings"]["energy_sources"]["gas"]["calories[k*W*h/m^3]"].as_f32().unwrap());


        // Costs

        input_values["buildings"]["costs"]["oil[M*eur/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["consumption"]["oil[M*L/a]"][sector].as_f32().unwrap()
            * constants["buildings"]["energy_sources"]["oil"]["price[eur/L]"].as_f32().unwrap());

        input_values["buildings"]["costs"]["gas[M*eur/a]"][sector] = json::JsonValue::from(
            input_values["buildings"]["consumption"]["gas[M*m^3/a]"][sector].as_f32().unwrap()
            * constants["buildings"]["energy_sources"]["gas"]["price[eur/m^3]"].as_f32().unwrap());


        // Invest

        input_values["buildings"]["invest"]["heat[eur]"][sector] = json::JsonValue::from(0);
        input_values["buildings"]["invest"]["thermal_energy_demand[eur]"][sector] = json::JsonValue::from(0);
        input_values["buildings"]["invest"]["grant_heat[]"][sector] = json::JsonValue::from(0);
        input_values["buildings"]["invest"]["grant_thermal_energy_demand[]"][sector] = json::JsonValue::from(0);

        if !prev_years_values.is_empty(){

            for heat_type in [
                "oil_no_condensing",
                "oil_with_condensing",
                "gas",
                "heat_pump",
                "other",
            ]{

                // invest heatings
                let area_this_year: f32 =
                    input_values["buildings"]["heat_types_areas"][&format!("{}[k*qm]", heat_type)][sector].as_f32().unwrap();
                let area_prev_year: f32 =
                    prev_years_values["buildings"]["heat_types_areas"][&format!("{}[k*qm]", heat_type)][sector].as_f32().unwrap();

                if area_this_year > area_prev_year{
                    input_values["buildings"]["invest"]["heat[eur]"][sector] = json::JsonValue::from(
                        input_values["buildings"]["invest"]["heat[eur]"][sector].as_f32().unwrap()
                        + (area_this_year - area_prev_year)
                        * constants["buildings"]["heat_types"][heat_type]["invest[eur*k*W*h/a]"].as_f32().unwrap());
                }

                // invest thermal energy demand
                let heat_demand_this_year: f32 =
                    input_values["buildings"]["demand"]["heat[k*W*h/qm/a]"][sector].as_f32().unwrap();
                let heat_demand_prev_year: f32 =
                    prev_years_values["buildings"]["demand"]["heat[k*W*h/qm/a]"][sector].as_f32().unwrap();

                if heat_demand_this_year > heat_demand_prev_year{
                    input_values["buildings"]["invest"]["thermal_energy_demand[eur]"][sector] = json::JsonValue::from(
                        input_values["buildings"]["invest"]["thermal_energy_demand[eur]"][sector].as_f32().unwrap()
                        + (heat_demand_this_year - heat_demand_prev_year)
                        * constants["buildings"]["energetic_restoration"]["invest[eur*k*W*h/a]"].as_f32().unwrap()
                        * input_values["buildings"]["base_data"]["floor_area[k*qm]"][sector].as_f32().unwrap());
                }

                // grant heatings
                if area_this_year > area_prev_year{
                    input_values["buildings"]["invest"]["grant_heat[]"][sector] = json::JsonValue::from(
                        input_values["buildings"]["invest"]["grant_heat[]"][sector].as_f32().unwrap()
                        + (area_this_year - area_prev_year)
                        * constants["buildings"]["heat_types"][heat_type]["invest[eur*k*W*h/a]"].as_f32().unwrap()
                        * constants["buildings"]["heat_types"][heat_type]["grant[]"].as_f32().unwrap());
                }

                // grant thermal energy demand
                if heat_demand_this_year > heat_demand_prev_year{

                    input_values["buildings"]["invest"]["thermal_energy_demand[eur]"][sector] = json::JsonValue::from(
                        input_values["buildings"]["invest"]["thermal_energy_demand[eur]"][sector].as_f32().unwrap()
                        + (heat_demand_this_year - heat_demand_prev_year)
                        * constants["buildings"]["energetic_restoration"]["invest[eur*k*W*h/a]"].as_f32().unwrap()
                        * input_values["buildings"]["base_data"]["floor_area[k*qm]"][sector].as_f32().unwrap());
                }
            }

        }
    }

    return input_values;
}
