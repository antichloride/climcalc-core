use crate::input::Input;
use crate::result::Results;
use std::ops;


pub struct SectorsInputs {
    private: Input,
    industry: Input,
    schools: Input,
    public: Input,
}

impl SectorsInputs{

    pub fn new(id: String, start_year: u32, end_year: u32) -> SectorsInputs{
        return SectorsInputs{
            private: Input::new((id.to_owned()+"/private").to_string(), start_year, end_year),
            industry: Input::new((id.to_owned()+"/industry").to_string(), start_year, end_year),
            schools: Input::new((id.to_owned()+"/schools").to_string(), start_year, end_year),
            public: Input::new((id.to_owned()+"/public").to_string(), start_year, end_year),
        }
    }

    pub fn get_year(&self, year: u32) -> SectorsRawValues{
        return SectorsRawValues{
            private: self.private.get_year(year),
            industry: self.industry.get_year(year),
            schools: self.schools.get_year(year),
            public: self.public.get_year(year),
        };
    }

    pub fn get_inputs(& self) -> Vec<&Input>{
        return Vec::from([&self.private, &self.industry, &self.schools, &self.public])
    }

    pub fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>{
        match id{
            "private" => Some(&mut self.private),
            "industry" => Some(&mut self.industry),
            "schools" => Some(&mut self.schools),
            "public" => Some(&mut self.public),
            _ => None,

        }

    }
}

pub struct SectorsRawValues {
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
impl ops::Add<SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn add(self, _rhs: SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private + _rhs.private,
            industry: self.industry + _rhs.industry,
            schools: self.schools + _rhs.schools,
            public: self.schools + _rhs.schools,
        }
    }
}

impl ops::Mul<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn mul(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private * _rhs.private,
            industry: self.industry * _rhs.industry,
            schools: self.schools * _rhs.schools,
            public: self.schools * _rhs.schools,
        }
    }
}
impl ops::Add<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn add(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private + _rhs.private,
            industry: self.industry + _rhs.industry,
            schools: self.schools + _rhs.schools,
            public: self.schools + _rhs.schools,
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
            impl ops::Mul<&SectorsRawValues> for $t{
                type Output = SectorsRawValues;
                fn mul(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self * _rhs.private as f32,
                        industry: self * _rhs.industry as f32,
                        schools: self * _rhs.schools as f32,
                        public: self * _rhs.schools as f32,
                    }
                }
            }
            impl ops::Mul<$t> for &SectorsRawValues{
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
            impl ops::Div<$t> for &SectorsRawValues{
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

pub struct SectorsResult{
    private: Results,
    industry: Results,
    schools: Results,
    public: Results,
}

impl SectorsResult{

    pub fn new(id: String, start_year: u32, end_year: u32) -> SectorsResult{
        return SectorsResult{
            private: Results::new((id.to_owned()+"/private").to_string(), start_year, end_year),
            industry: Results::new((id.to_owned()+"/industry").to_string(), start_year, end_year),
            schools: Results::new((id.to_owned()+"/schools").to_string(), start_year, end_year),
            public: Results::new((id.to_owned()+"/public").to_string(), start_year, end_year),
        }
    }

    pub fn get_year(&self, year: u32) -> SectorsRawValues{
        return SectorsRawValues{
            private: self.private.get_year(year),
            industry: self.industry.get_year(year),
            schools: self.schools.get_year(year),
            public: self.public.get_year(year),
        };
    }

    pub fn set_year_values(&mut self, year: u32, values: &SectorsRawValues){
        self.private.set_year_value(year, values.private);
        self.industry.set_year_value(year, values.industry);
        self.schools.set_year_value(year, values.schools);
        self.public.set_year_value(year, values.public);
    }

    pub fn get_results(& self) -> Vec<&Results>{
        return Vec::from([&self.private, &self.industry, &self.schools, &self.public])
    }

    pub fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
        match id{
            "private" => Some(&mut self.private),
            "industry" => Some(&mut self.industry),
            "schools" => Some(&mut self.schools),
            "public" => Some(&mut self.public),
            _ => None,

        }

    }
}
