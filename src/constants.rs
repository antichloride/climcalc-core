
pub mod buildings{
    pub struct HeatType {
        pub efficency: f32,
        pub invest: f32,
        pub grant: f32,
    }

    pub const oil_no_condensing: HeatType = HeatType {
        efficency: 0.7,
        invest: 0.5,
        grant: 0.0,
    };
    pub const oil_with_condensing: HeatType = HeatType {
        efficency: 0.9,
        invest: 1.0,
        grant: 0.1,
    };
    pub const gas: HeatType = HeatType {
        efficency: 0.9,
        invest: 0.5,
        grant: 0.2,
    };
    pub const heat_pump: HeatType = HeatType {
        efficency: 3.0,
        invest: 2.0,
        grant: 0.3,
    };
    pub const other: HeatType = HeatType {
        efficency: 0.5,
        invest: 0.0,
        grant: 0.5,
    };

    pub mod EnergySource{

        pub struct EnergySource {
            pub calories: f32,
            pub price: f32,
            pub emission: f32,
        }

        pub const oil: EnergySource = EnergySource{
            calories: 10.0,
            price: 0.9,
            emission: 3.0,
        };

        pub const gas: EnergySource = EnergySource{
            calories: 5.0,
            price: 0.9,
            emission: 2.0,
        };

        pub const other: EnergySource = EnergySource{
            calories: 0.0,
            price: 0.9,
            emission: 0.0,
        };

    }

    pub mod energetic_restoration{
        pub const invest: f32 = 2.5;
        pub const grant: f32 = 0.3;
    }



}

pub mod mobility{

        pub const price_fuel: f32 = 1.7;

        pub struct Vehicle {
            pub price: f32,
            pub grant: f32,
            pub consumption: f32,
            pub fuel_costs: f32,
            pub operation_costs: f32,
            pub taxes_and_checks: f32,
            pub lifecycle: f32,
        }

        pub const bev: Vehicle = Vehicle{
            price: 35.0,
            grant: 9.0,
            consumption: 24.0,
            fuel_costs: 0.3,
            operation_costs: 0.05,
            taxes_and_checks: 1.0,
            lifecycle: 10.0,
        };


        pub const combustor: Vehicle = Vehicle{
            price: 30.0,
            grant: 0.0,
            consumption: 70.0,
            fuel_costs: 0.17,
            operation_costs: 0.05,
            taxes_and_checks: 1.0,
            lifecycle: 10.0,
        };
}

pub mod energy{

    pub struct RenewableEnergy{
        pub invest: f32,
        pub grant: f32,
        pub operation_costs: f32,
        pub lifecycle: f32,
        pub peak_power_to_mean_power: f32,
        pub buyback_price: f32,
        pub power_per_area: f32,
        pub costs: f32,
    }

    pub const solar_roof: RenewableEnergy = RenewableEnergy{
        invest: 1500.0,
        grant: 500.0,
        operation_costs: 0.02,
        lifecycle: 30.0,
        peak_power_to_mean_power: 900.0,
        buyback_price: 0.07,
        power_per_area: 0.17,
        costs: 0.07,
    };

    pub const solar_landscape: RenewableEnergy = RenewableEnergy{
        invest: 1200.0,
        grant: 500.0,
        operation_costs: 0.02,
        lifecycle: 30.0,
        peak_power_to_mean_power: 900.0,
        buyback_price: 0.07,
        power_per_area: 0.17,
        costs: 0.05,
    };

    pub const wind_onshore: RenewableEnergy = RenewableEnergy{
        invest: 2000.0,
        grant: 750.0,
        operation_costs: 0.02,
        lifecycle: 20.0,
        peak_power_to_mean_power: 1300.0,
        buyback_price: 0.0,
        power_per_area: 0.0,
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
