#![allow(non_upper_case_globals, non_snake_case, dead_code)]

pub mod buildings{
    pub struct HeatType {
        pub efficency: f32,
        pub invest__m__eur_per_W_h: f32,
        pub grant: f32,
    }

    pub const oil_no_condensing: HeatType = HeatType {
        efficency: 0.87,
        invest__m__eur_per_W_h: 0.5,
        grant: 0.0,
    };
    pub const oil_with_condensing: HeatType = HeatType {
        efficency: 0.98,
        invest__m__eur_per_W_h: 1.0,
        grant: 0.1,
    };
    pub const gas: HeatType = HeatType {
        efficency: 0.93,
        invest__m__eur_per_W_h: 0.5,
        grant: 0.2,
    };
    pub const heat_pump: HeatType = HeatType {
        efficency: 3.0,
        invest__m__eur_per_W_h: 2.0,
        grant: 0.3,
    };
    pub const other: HeatType = HeatType {
        efficency: 1.0,
        invest__m__eur_per_W_h: 1.0,
        grant: 0.5,
    };

    pub mod EnergySource{

        pub mod oil{
            pub const energy_density__k__W_h_per_L: f32 = 9.0;
            pub const price__eur_per_L: f32 = 0.82;
            pub const emission__kg_coe_per_L: f32 = 3.0;
        }

        pub mod gas{
            pub const energy_density__k__W_h_per_m3: f32 = 11.0;
            pub const price__eur_per_m3: f32 = 0.73;
            pub const emission__kg_coe_per_m3: f32 = 2.0;
        }

        pub mod other{
            pub const energy_density: f32 = 0.0;
            pub const price: f32 = 0.82;
            pub const emission: f32 = 0.0;
        }

    }

    pub mod energetic_restoration{
        pub const invest__m__eur_per_W_h_m2: f32 = 2.5;
        pub const grant: f32 = 0.3;
    }



}

pub mod mobility{

        pub const price_fuel: f32 = 1.7;

        pub mod bev{
            pub const price: f32 = 35.0;
            pub const grant_per_car__k__eur: f32 = 9.0;
            pub const nrg_cnsmp__1em2__W_h_per_m: f32 = 16.0;
            pub const om_costs: f32 = 0.05;
            pub const fuel_costs: f32 = 0.05;
            pub const taxes_and_checks: f32 = 1.0;
            pub const lifecycle: f32 = 10.0;
        }

        pub mod combustor{
            pub const price: f32 = 30.0;
            pub const grant_per_car__k__eur: f32 = 0.0;
            pub const nrg_cnsmp__1em2__W_h_per_m: f32 = 70.0;
            pub const fuel_costs: f32 = 0.07;
            pub const om_costs: f32 = 0.05;
            pub const taxes_and_checks: f32 = 1.0;
            pub const lifecycle: f32 = 10.0;
        }

}

pub mod energy{

    pub struct RenewableEnergy{
        pub invest__m__eur_per_Wp: f32,
        pub grant__m__eur_per_Wp: f32,
        pub operation_costs: f32,
        pub lifecycle: f32,
        pub Wp_to_W_h_per_a: f32,
        pub buyback_price__m__eur_per_W_h: f32,
        pub power_per_area__k__Wp_per_m2: f32,
        pub costs: f32,
    }

    pub const solar_roof: RenewableEnergy = RenewableEnergy{
        invest__m__eur_per_Wp: 1500.0,
        grant__m__eur_per_Wp: 500.0,
        operation_costs: 0.02,
        lifecycle: 30.0,
        Wp_to_W_h_per_a: 900.0,
        buyback_price__m__eur_per_W_h: 0.07,
        power_per_area__k__Wp_per_m2: 0.17,
        costs: 0.07,
    };

    pub const solar_landscape: RenewableEnergy = RenewableEnergy{
        invest__m__eur_per_Wp: 1200.0,
        grant__m__eur_per_Wp: 500.0,
        operation_costs: 0.02,
        lifecycle: 30.0,
        Wp_to_W_h_per_a: 900.0,
        buyback_price__m__eur_per_W_h: 0.07,
        power_per_area__k__Wp_per_m2: 0.17,
        costs: 0.05,
    };

    pub const wind_onshore: RenewableEnergy = RenewableEnergy{
        invest__m__eur_per_Wp: 2000.0,
        grant__m__eur_per_Wp: 750.0,
        operation_costs: 0.02,
        lifecycle: 20.0,
        Wp_to_W_h_per_a: 1300.0,
        buyback_price__m__eur_per_W_h: 0.0,
        power_per_area__k__Wp_per_m2: 0.0,
        costs: 0.08,
    };

    pub mod evu_power_mix{
        pub const coal: f32 = 0.297;
        pub const gas: f32 = 0.105;
        pub const atom: f32 = 0.133;
        pub const pv: f32 = 0.099;
        pub const wind: f32 = 0.230;
        pub const other: f32 = 13.6;
    }

    pub mod evu_emissions{
        pub const coal: f32 = 1000.0;
        pub const gas: f32 = 600.0;
    }

    pub const evu_discount_heat_pump: f32 = 0.2;

}

pub mod economy{

    pub const revenue_margin: f32 = 0.05;

    pub mod material{
        pub mod invest_local{
            pub const heating_heatpump: f32 = 0.5;
            pub const energetic_restoration: f32 = 0.3;
            pub const fast_charging_stations: f32 = 0.65;
            pub const solar_roof: f32 = 0.5;
            pub const solar_landscape: f32 = 0.65;
        }
        pub mod invest_national{
            pub const heating_heatpump: f32 = 0.5;
            pub const energetic_restoration: f32 = 0.3;
            pub const fast_charging_stations: f32 = 0.65;
            pub const solar_roof: f32 = 0.5;
            pub const solar_landscape: f32 = 0.65;
        }
    }
    pub mod invest_work{
        pub mod loacal_value_add{
            pub const heating_heatpump: f32 = 0.3;
            pub const energetic_restoration: f32 = 0.3;
            pub const fast_charging_stations: f32 = 0.3;
            pub const solar_roof: f32 = 0.3;
            pub const solar_landscape: f32 = 0.15;
            pub const community: f32 = 0.5;
        }
        pub mod national_added_value{
            pub const heating_heatpump: f32 = 1.0;
            pub const energetic_restoration: f32 = 1.0;
            pub const fast_charging_stations: f32 = 1.0;
            pub const solar_roof: f32 = 1.0;
            pub const solar_landscape: f32 = 1.0;
            pub const community: f32 = 1.0;
        }
    }
    pub mod fte_per_million_turnover{
        pub mod material{
            pub const heating_heatpump: f32 = 10.0;
            pub const energetic_restoration: f32 = 10.0;
            pub const fast_charging_stations: f32 = 10.0;
            pub const solar_roof: f32 = 10.0;
            pub const solar_landscape: f32 = 10.0;
        }
        pub mod work{
            pub const heating_heatpump: f32 = 20.0;
            pub const energetic_restoration: f32 = 20.0;
            pub const fast_charging_stations: f32 = 20.0;
            pub const solar_roof: f32 = 20.0;
            pub const solar_landscape: f32 = 20.0;
        }
    }
    pub mod maintenance_community_costs{
        pub mod loacal_value_add{
            pub const fast_charging_stations: f32 = 0.3;
            pub const solar_roof: f32 = 0.3;
            pub const solar_landscape: f32 = 0.2;
            pub const community: f32 = 0.75;
        }
        pub mod national_added_value{
            pub const fast_charging_stations: f32 = 1.0;
            pub const solar_roof: f32 = 1.0;
            pub const solar_landscape: f32 = 1.0;
            pub const community: f32 = 0.75;
        }
    }

    pub mod tax{
        pub const business_tax: f32 = 0.008;
        pub const income_per_fte: f32 = 50.0;
        pub const income_tax_per_fte: f32 = 7.5;
        pub const income_tax_local_per_fte: f32 = 1.1;
        pub const turnover_tax: f32 = 0.19;
        pub const turnover_tax_local_part: f32 = 0.004;
        pub const corporate_tax: f32 = 0.15;
    }
}
