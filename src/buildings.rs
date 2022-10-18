
pub mod buildings{

    use wasm_bindgen::prelude::*;
    use std::ptr;
    use std::ops;

    pub struct Buildings {
        inputs: InputsBuildings,
        results: ResultsBuildings,
    }

    impl Buildings{

        pub fn new(start_year: u32, end_year: u32) -> Buildings{
            return Buildings{
                inputs: InputsBuildings::new("buildings/inputs", start_year, end_year),
                results: ResultsBuildings::new(start_year, end_year),
            }
        }

        pub fn calculate(&mut self, year: u32){
            let inputs = &self.inputs;
            let results = &mut self.results;

            let floor_area_per_building = self.inputs.floor_area_per_building.get_year(year);
            let n_buildings = self.inputs.n_buildings.get_year(year);
            let floor_area = floor_area_per_building * n_buildings * 1e-3;

            results.floor_area.set_year_values(year, floor_area);
        }

        pub fn get_inputs(& self) -> Vec<&Input>{
            return self.inputs.get_inputs();
        }
    }


    macro_rules! implement_over_sectors{
        ($($sector:ident),*) => {

             = Sectors{
                $(
                    $sector: self.buildings.floor_area_per_building.$sector.values[year_index]
                * self.buildings.n_buildings.$sector.values[year_index] / 1e3,
                 )*
            };
        }
    }

    struct InputsBuildings{
        n_buildings: SectorsInputs,
        floor_area_per_building: SectorsInputs,
        heat_demand_per_area_per_year: SectorsInputs,
    }

    impl InputsBuildings{
        fn new(id: &str, start_year: u32, end_year: u32) -> InputsBuildings {
            return InputsBuildings{
                n_buildings: SectorsInputs::new(id.to_owned()+"/n_buildings", start_year, end_year),
                floor_area_per_building: SectorsInputs::new(id.to_owned()+"/floor_area_per_building", start_year, end_year),
                heat_demand_per_area_per_year: SectorsInputs::new(id.to_owned()+"/heat_demand_per_area_per_year", start_year, end_year),
            }
        }

        fn get_inputs(& self) -> Vec<&Input>{
            let mut inputs = self.n_buildings.get_inputs();
            inputs.extend(self.floor_area_per_building.get_inputs());
            return inputs
        }
    }

    struct ResultsBuildings{
        floor_area: SectorsResult,
    }

    impl ResultsBuildings{
        fn new(start_year: u32, end_year: u32) -> ResultsBuildings{
            return ResultsBuildings{
                floor_area: SectorsResult::new(start_year, end_year),
            }
        }
    }

    struct SectorsInputs {
        private: Input,
        industry: Input,
        schools: Input,
        public: Input,
    }

    impl SectorsInputs{

        fn new(id: String, start_year: u32, end_year: u32) -> SectorsInputs{
            return SectorsInputs{
                private: Input::new((id.to_owned()+"/private").to_string(), start_year, end_year),
                industry: Input::new((id.to_owned()+"/industry").to_string(), start_year, end_year),
                schools: Input::new((id.to_owned()+"/schools").to_string(), start_year, end_year),
                public: Input::new((id.to_owned()+"/public").to_string(), start_year, end_year),
            }
        }

        fn get_year(&self, year: u32) -> SectorsRawValues{
            return SectorsRawValues{
                private: self.private.get_year(year),
                industry: self.industry.get_year(year),
                schools: self.schools.get_year(year),
                public: self.public.get_year(year),
            };
        }
        fn get_inputs(& self) -> Vec<&Input>{
            return Vec::from([&self.private, &self.industry, &self.schools, &self.public])
        }
    }

    struct SectorsRawValues {
        private: f32,
        industry: f32,
        schools: f32,
        public: f32,
    }

    impl ops::Mul<SectorsRawValues> for SectorsRawValues{
        type Output = SectorsRawValues;
        fn mul(self, _rhs: SectorsRawValues) -> SectorsRawValues {
            SectorsRawValues {
                private: self.private * _rhs.private,
                industry: self.industry * _rhs.industry,
                schools: self.schools * _rhs.schools,
                public: self.schools * _rhs.schools,
            }
        }
    }

    macro_rules! impl_math_ops_for_sector {
        ($($t:ty),*) => {
            $(
                impl ops::Mul<SectorsRawValues> for $t{
                    type Output = SectorsRawValues;
                    fn mul(self, _rhs: SectorsRawValues) -> SectorsRawValues {
                        SectorsRawValues {
                            private: self * _rhs.private as f32,
                            industry: self * _rhs.industry as f32,
                            schools: self * _rhs.schools as f32,
                            public: self * _rhs.schools as f32,
                        }
                    }
                }
                impl ops::Mul<$t> for SectorsRawValues{
                    type Output = SectorsRawValues;
                    fn mul(self, _rhs: $t) -> SectorsRawValues {
                        SectorsRawValues {
                            private: self.private * _rhs as f32,
                            industry: self.industry * _rhs as f32,
                            schools: self.schools * _rhs as f32,
                            public: self.schools * _rhs as f32,
                        }
                    }
                }
                impl ops::Div<$t> for SectorsRawValues{
                    type Output = SectorsRawValues;
                    fn div(self, _rhs: $t) -> SectorsRawValues {
                        SectorsRawValues {
                            private: self.private / _rhs as f32,
                            industry: self.industry / _rhs as f32,
                            schools: self.schools / _rhs as f32,
                            public: self.schools / _rhs as f32,
                        }
                    }
                }
            )*
        }
    }

    impl_math_ops_for_sector!{f32}

    struct SectorsResult{
        private: Results,
        industry: Results,
        schools: Results,
        public: Results,
    }

    impl SectorsResult{

        fn new(start_year: u32, end_year: u32) -> SectorsResult{
            return SectorsResult{
                private: Results::new(start_year, end_year),
                industry: Results::new(start_year, end_year),
                schools: Results::new(start_year, end_year),
                public: Results::new(start_year, end_year),
            }
        }

        fn set_year_values(&mut self, year: u32, values: SectorsRawValues){
            self.private.set_year_value(year, values.private);
            self.industry.set_year_value(year, values.industry);
            self.schools.set_year_value(year, values.schools);
            self.public.set_year_value(year, values.public);
        }
    }

    struct Results{
        values: Vec<f32>,
        start_year: u32,
        end_year: u32,
    }

    impl Results{

        fn new(start_year: u32, end_year: u32) -> Results{
            return Results{
                values: vec![0.0; (end_year-start_year+1) as usize],
                start_year: start_year,
                end_year: start_year,
            }
        }

        fn set_year_value(&mut self, year: u32, value: f32){
            let year_index = (year - self.start_year) as usize;
            self.values[year_index] = value;
        }
    }

    #[wasm_bindgen]
    pub struct Input{
        id: String,
        values: Vec<f32>,
        measures: Vec<Measure>,
        start_year: u32,
        end_year: u32,
    }

    #[wasm_bindgen]
    impl Input{

        fn new(id: String, start_year: u32, end_year: u32) -> Input{
            return Input{
                id: id,
                values: vec![0.0; (end_year-start_year+1) as usize],
                measures: Vec::from([]),
                start_year: start_year,
                end_year: start_year,
            }
        }

        fn get_year(&self, year: u32) -> f32{
            return self.values[(year-self.start_year) as usize];
        }
        fn apply_measures(mut self) {
            for year_index in 0..self.start_year-self.end_year {
                for measure in &self.measures {
                    self.values[year_index as usize] +=
                       measure.delta_per_year(year_index + self.start_year);
                }
            }
        }

        fn get_measure_index(&self, measure: &Measure) -> usize{
            return self.measures.iter().position(|m| ptr::eq(&m, &measure)).unwrap();
        }
    }

    struct Measure {
        id: u32,
        affected_input: Input,
        start_year: u32,
        end_year: u32,
        delta: f32,
    }

    impl Measure{
        fn update(mut self) {
            self.affected_input.apply_measures();
        }

        fn delta_per_year(&self, year: u32) -> f32{
            // start_yaer is the first yaer in which the measure is applied
            // end year is the last year in which the measures is applied
            // the target delta is reached at the end of the end_year
            if year < self.start_year || year > self.end_year{
                return 0.0;
            }
            return self.delta / (self.start_year - self.end_year + 1) as f32
        }

        fn delete(&mut self) {
            let index = self.affected_input.get_measure_index(&self);
            self.affected_input.measures.remove(index);
        }
    }


    #[cfg(test)]
    mod tests{
        use super::*;
        extern crate wasm_bindgen_test;
        use wasm_bindgen_test::*;
        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        fn test_input_and_measure() {
            let start_year = 2022 as u32;
            let end_year = 2025 as u32;
            let input = Input::new(start_year, end_year);

            assert_eq!(input.values, Vec::from([0.0,0.0,0.0,0.0]));
        }
    }

}

