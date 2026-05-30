use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Soil
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soil {
    pub nitrogen: f64,
    pub phosphorus: f64,
    pub potassium: f64,
    pub moisture: f64,
    pub ph: f64,
    pub organic_matter: f64,
}

impl Soil {
    pub fn new() -> Self {
        Self {
            nitrogen: 50.0,
            phosphorus: 50.0,
            potassium: 50.0,
            moisture: 50.0,
            ph: 6.5,
            organic_matter: 30.0,
        }
    }

    /// Weighted average fertility score (0–100).
    pub fn fertility(&self) -> f64 {
        let nutrient_score = (self.nitrogen + self.phosphorus + self.potassium) / 3.0;
        // pH sweet spot: 6.0–7.5
        let ph_score = 100.0 - (self.ph - 6.75).abs() * 40.0;
        let ph_score = ph_score.clamp(0.0, 100.0);
        let moisture_score = if self.moisture < 20.0 {
            self.moisture * 5.0
        } else if self.moisture > 80.0 {
            100.0 - (self.moisture - 80.0) * 2.5
        } else {
            100.0
        };
        let om_score = (self.organic_matter * 2.0).min(100.0);

        nutrient_score * 0.35 + ph_score * 0.15 + moisture_score * 0.25 + om_score * 0.25
    }

    /// Remove nutrients (farming consumes resources).
    pub fn deplete(&mut self, amounts: &Soil) {
        self.nitrogen = (self.nitrogen - amounts.nitrogen).max(0.0);
        self.phosphorus = (self.phosphorus - amounts.phosphorus).max(0.0);
        self.potassium = (self.potassium - amounts.potassium).max(0.0);
        self.moisture = (self.moisture - amounts.moisture).max(0.0);
        self.organic_matter = (self.organic_matter - amounts.organic_matter).max(0.0);
    }

    /// Add nutrients (compost / fertilizer).
    pub fn replenish(&mut self, amounts: &Soil) {
        self.nitrogen = (self.nitrogen + amounts.nitrogen).min(100.0);
        self.phosphorus = (self.phosphorus + amounts.phosphorus).min(100.0);
        self.potassium = (self.potassium + amounts.potassium).min(100.0);
        self.moisture = (self.moisture + amounts.moisture).min(100.0);
        self.organic_matter = (self.organic_matter + amounts.organic_matter).min(100.0);
    }

    /// Natural regeneration each tick: slight nitrogen fixation, organic matter decay, pH drift toward neutral.
    pub fn tick(&mut self) {
        self.nitrogen = (self.nitrogen + 0.02).min(100.0);
        self.organic_matter = (self.organic_matter - 0.005).max(0.0);
        // pH drifts toward 7.0
        self.ph += (7.0 - self.ph) * 0.001;
        // Moisture evaporates slowly
        self.moisture = (self.moisture - 0.01).max(0.0);
    }

    fn clamp_all(&mut self) {
        self.nitrogen = self.nitrogen.clamp(0.0, 100.0);
        self.phosphorus = self.phosphorus.clamp(0.0, 100.0);
        self.potassium = self.potassium.clamp(0.0, 100.0);
        self.moisture = self.moisture.clamp(0.0, 100.0);
        self.ph = self.ph.clamp(0.0, 14.0);
        self.organic_matter = self.organic_matter.clamp(0.0, 100.0);
    }
}

impl Default for Soil {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// CropType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CropType {
    Wheat,
    Corn,
    Potato,
    Carrot,
    Tomato,
    Sunflower,
    Bean,
    Rice,
    Pumpkin,
    Lavender,
}

impl CropType {
    /// Nutrients consumed per growth stage.
    pub fn nutrient_need(&self) -> Soil {
        match self {
            Self::Wheat => Soil { nitrogen: 1.2, phosphorus: 0.5, potassium: 0.8, moisture: 1.0, ph: 0.0, organic_matter: 0.1 },
            Self::Corn => Soil { nitrogen: 2.0, phosphorus: 0.8, potassium: 1.2, moisture: 1.5, ph: 0.0, organic_matter: 0.15 },
            Self::Potato => Soil { nitrogen: 0.8, phosphorus: 0.6, potassium: 1.5, moisture: 1.2, ph: 0.0, organic_matter: 0.1 },
            Self::Carrot => Soil { nitrogen: 0.5, phosphorus: 0.4, potassium: 0.8, moisture: 1.0, ph: 0.0, organic_matter: 0.05 },
            Self::Tomato => Soil { nitrogen: 1.5, phosphorus: 0.7, potassium: 1.3, moisture: 1.8, ph: 0.0, organic_matter: 0.12 },
            Self::Sunflower => Soil { nitrogen: 1.0, phosphorus: 0.6, potassium: 1.0, moisture: 1.2, ph: 0.0, organic_matter: 0.08 },
            Self::Bean => Soil { nitrogen: 0.2, phosphorus: 0.5, potassium: 0.6, moisture: 1.0, ph: 0.0, organic_matter: 0.1 },
            Self::Rice => Soil { nitrogen: 1.3, phosphorus: 0.5, potassium: 1.0, moisture: 2.5, ph: 0.0, organic_matter: 0.1 },
            Self::Pumpkin => Soil { nitrogen: 1.8, phosphorus: 0.9, potassium: 1.4, moisture: 1.6, ph: 0.0, organic_matter: 0.15 },
            Self::Lavender => Soil { nitrogen: 0.3, phosphorus: 0.3, potassium: 0.4, moisture: 0.6, ph: 0.0, organic_matter: 0.05 },
        }
    }

    /// Ticks to full maturity.
    pub fn growth_time(&self) -> u64 {
        match self {
            Self::Wheat => 60,
            Self::Corn => 90,
            Self::Potato => 70,
            Self::Carrot => 50,
            Self::Tomato => 80,
            Self::Sunflower => 75,
            Self::Bean => 55,
            Self::Rice => 85,
            Self::Pumpkin => 100,
            Self::Lavender => 65,
        }
    }

    /// Harvest output units.
    pub fn yield_amount(&self) -> u32 {
        match self {
            Self::Wheat => 8,
            Self::Corn => 6,
            Self::Potato => 10,
            Self::Carrot => 12,
            Self::Tomato => 9,
            Self::Sunflower => 5,
            Self::Bean => 14,
            Self::Rice => 10,
            Self::Pumpkin => 4,
            Self::Lavender => 7,
        }
    }

    /// Legumes and lavender fix atmospheric nitrogen.
    pub fn adds_nitrogen(&self) -> bool {
        matches!(self, Self::Bean | Self::Lavender)
    }

    /// Water requirement per tick (0–100 scale).
    pub fn water_need(&self) -> f64 {
        match self {
            Self::Wheat => 40.0,
            Self::Corn => 55.0,
            Self::Potato => 45.0,
            Self::Carrot => 35.0,
            Self::Tomato => 60.0,
            Self::Sunflower => 40.0,
            Self::Bean => 35.0,
            Self::Rice => 80.0,
            Self::Pumpkin => 55.0,
            Self::Lavender => 20.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Crop
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub crop_type: CropType,
    pub growth_stage: f64,
    pub health: f64,
    pub planted_tick: u64,
}

impl Crop {
    pub fn new(crop_type: CropType, planted_tick: u64) -> Self {
        Self {
            crop_type,
            growth_stage: 0.0,
            health: 1.0,
            planted_tick,
        }
    }

    /// Advance growth based on soil, water, and sunlight (0–1).
    pub fn tick(&mut self, soil: &Soil, water: f64, sunlight: f64) {
        let fertility = soil.fertility() / 100.0;
        let water_factor = if water >= self.crop_type.water_need() {
            1.0
        } else {
            water / self.crop_type.water_need()
        };
        let sunlight_factor = sunlight.clamp(0.0, 1.0);

        // Growth increment per tick
        let increment = fertility * 0.4 + water_factor * 0.3 + sunlight_factor * 0.3;
        let base_increment = 1.0 / self.crop_type.growth_time() as f64;
        self.growth_stage = (self.growth_stage + base_increment * increment).min(1.0);

        // Health degrades if conditions are poor
        let stress = (1.0 - water_factor) * 0.3 + (1.0 - sunlight_factor) * 0.1 + (1.0 - fertility) * 0.2;
        self.health = (self.health - stress * 0.01).max(0.0);

        // Minimum health recovery when conditions are good
        if water_factor > 0.8 && sunlight_factor > 0.5 {
            self.health = (self.health + 0.002).min(1.0);
        }
    }

    pub fn is_mature(&self) -> bool {
        self.growth_stage >= 1.0
    }

    /// Harvest: returns yield scaled by health, resets growth.
    pub fn harvest(&mut self) -> u32 {
        if !self.is_mature() {
            return 0;
        }
        let yield_base = self.crop_type.yield_amount();
        let actual = (yield_base as f64 * self.health).round() as u32;
        self.growth_stage = 0.0;
        self.health = 1.0;
        actual
    }
}

// ---------------------------------------------------------------------------
// Weather
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub rainfall: f64,
    pub sunlight: f64,
    pub temperature: f64,
}

impl Weather {
    pub fn ideal() -> Self {
        Self {
            rainfall: 50.0,
            sunlight: 0.8,
            temperature: 22.0,
        }
    }
}

// ---------------------------------------------------------------------------
// FarmPlot
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmPlot {
    pub soil: Soil,
    pub crop: Option<Crop>,
    pub size: f64,
}

impl FarmPlot {
    pub fn new(size: f64) -> Self {
        Self {
            soil: Soil::new(),
            crop: None,
            size: size.max(0.1),
        }
    }

    pub fn tick(&mut self, weather: &Weather, _total_ticks: u64) {
        self.soil.tick();

        // Rainfall adds moisture
        self.soil.moisture = (self.soil.moisture + weather.rainfall * 0.02).min(100.0);

        if let Some(ref mut crop) = self.crop {
            crop.tick(&self.soil, self.soil.moisture, weather.sunlight);

            // Crop consumes nutrients
            let need = crop.crop_type.nutrient_need();
            let scaled = Soil {
                nitrogen: need.nitrogen * self.size,
                phosphorus: need.phosphorus * self.size,
                potassium: need.potassium * self.size,
                moisture: need.moisture * self.size,
                ph: 0.0,
                organic_matter: need.organic_matter * self.size,
            };
            self.soil.deplete(&scaled);

            // Nitrogen fixers
            if crop.crop_type.adds_nitrogen() {
                self.soil.nitrogen = (self.soil.nitrogen + 0.15).min(100.0);
            }

            // Crop dies if health hits zero
            if crop.health <= 0.0 {
                self.soil.organic_matter = (self.soil.organic_matter + 1.0).min(100.0);
                self.crop = None;
            }
        }

        self.soil.clamp_all();
    }
}

// ---------------------------------------------------------------------------
// Animal
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Animal {
    Chicken,
    Cow,
    Sheep,
    Pig,
    Bee,
    Horse,
}

impl Animal {
    pub fn product(&self) -> Option<FarmProduct> {
        match self {
            Self::Chicken => Some(FarmProduct::Egg(1)),
            Self::Cow => Some(FarmProduct::Milk(4)),
            Self::Sheep => Some(FarmProduct::Wool(2)),
            Self::Bee => Some(FarmProduct::Honey(1)),
            Self::Pig => Some(FarmProduct::Manure(1.5)),
            Self::Horse => None,
        }
    }

    pub fn feed_cost(&self) -> f64 {
        match self {
            Self::Chicken => 1.0,
            Self::Cow => 4.0,
            Self::Sheep => 2.5,
            Self::Pig => 3.0,
            Self::Bee => 0.5,
            Self::Horse => 3.5,
        }
    }

    pub fn breeding_threshold(&self) -> f64 {
        match self {
            Self::Chicken => 0.5,
            Self::Cow => 0.7,
            Self::Sheep => 0.6,
            Self::Pig => 0.55,
            Self::Bee => 0.4,
            Self::Horse => 0.75,
        }
    }
}

// ---------------------------------------------------------------------------
// FarmProduct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FarmProduct {
    Egg(u32),
    Milk(u32),
    Wool(u32),
    Honey(u32),
    Manure(f64),
}

// ---------------------------------------------------------------------------
// Farm
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Farm {
    pub plots: Vec<FarmPlot>,
    pub livestock: Vec<Animal>,
    pub silo: HashMap<CropType, u32>,
    pub water_supply: f64,
    pub compost: f64,
    pub total_ticks: u64,
}

impl Farm {
    pub fn new(num_plots: usize) -> Self {
        let plots = (0..num_plots).map(|_| FarmPlot::new(1.0)).collect();
        Self {
            plots,
            livestock: Vec::new(),
            silo: HashMap::new(),
            water_supply: 100.0,
            compost: 50.0,
            total_ticks: 0,
        }
    }

    /// Advance the entire farm by one tick.
    pub fn tick(&mut self, weather: &Weather) {
        self.total_ticks += 1;

        // Tick all plots
        for plot in &mut self.plots {
            plot.tick(weather, self.total_ticks);
        }

        // Livestock consume from silo
        let mut feed_needed: f64 = self.livestock.iter().map(|a| a.feed_cost()).sum();
        // Try to feed from silo
        let silo_total: u32 = self.silo.values().sum();
        if (silo_total as f64) >= feed_needed {
            // Deduct feed proportionally
            if silo_total > 0 {
                let ratio = feed_needed / silo_total as f64;
                let keys: Vec<CropType> = self.silo.keys().copied().collect();
                for k in keys {
                    if let Some(v) = self.silo.get_mut(&k) {
                        *v = (*v as f64 * (1.0 - ratio)).round() as u32;
                    }
                }
            }
            feed_needed = 0.0;
        } else {
            // Consume all silo, rest unfed
            for v in self.silo.values_mut() {
                feed_needed -= *v as f64;
                *v = 0;
            }
        }

        // Animals produce and add manure to compost
        for animal in &self.livestock {
            if feed_needed <= 0.0 {
                if let Some(FarmProduct::Manure(amount)) = animal.product() {
                    self.compost += amount * 0.1;
                }
            }
        }

        // Rainfall adds to water supply
        self.water_supply = (self.water_supply + weather.rainfall * 0.05).min(200.0);

        // Water is consumed by plots
        let water_per_plot = 0.5;
        let total_water = water_per_plot * self.plots.len() as f64;
        self.water_supply = (self.water_supply - total_water).max(0.0);

        // Clean silo
        self.silo.retain(|_, v| *v > 0);
    }

    /// Plant a crop in the first available plot.
    pub fn plant(&mut self, crop: CropType) -> bool {
        for plot in &mut self.plots {
            if plot.crop.is_none() {
                plot.crop = Some(Crop::new(crop, self.total_ticks));
                return true;
            }
        }
        false
    }

    /// Harvest all mature crops, return total yield.
    pub fn harvest_mature(&mut self) -> u32 {
        let mut total = 0;
        for plot in &mut self.plots {
            if let Some(ref mut crop) = plot.crop {
                if crop.is_mature() {
                    let yield_val = crop.harvest();
                    if yield_val > 0 {
                        *self.silo.entry(crop.crop_type).or_insert(0) += yield_val;
                        total += yield_val;
                        // Add organic matter to soil after harvest
                        plot.soil.organic_matter = (plot.soil.organic_matter + 0.5).min(100.0);
                    }
                    // Reset crop for replanting
                    if crop.growth_stage == 0.0 && yield_val > 0 {
                        plot.crop = None;
                    }
                }
            }
        }
        total
    }

    /// Add compost to all plots.
    pub fn add_compost(&mut self, amount: f64) {
        self.compost += amount;
        let per_plot = amount / self.plots.len().max(1) as f64;
        let compost_nutrients = Soil {
            nitrogen: per_plot * 2.0,
            phosphorus: per_plot * 1.5,
            potassium: per_plot * 1.0,
            moisture: per_plot * 0.5,
            ph: 0.0,
            organic_matter: per_plot * 3.0,
        };
        for plot in &mut self.plots {
            plot.soil.replenish(&compost_nutrients);
        }
        self.compost = (self.compost - amount).max(0.0);
    }

    /// Average fertility across all plots.
    pub fn soil_health(&self) -> f64 {
        if self.plots.is_empty() {
            return 0.0;
        }
        self.plots.iter().map(|p| p.soil.fertility()).sum::<f64>() / self.plots.len() as f64
    }

    /// Conservation error: deviation from sustainable resource balance.
    /// Returns 0.0 when perfectly balanced, higher values indicate imbalance.
    pub fn conservation_error(&self) -> f64 {
        let soil_err: f64 = self
            .plots
            .iter()
            .map(|p| {
                let f = p.soil.fertility();
                (f - 50.0).abs() / 50.0 // deviation from ideal mid-range
            })
            .sum::<f64>()
            / self.plots.len().max(1) as f64;

        let water_err = if self.water_supply > 150.0 {
            (self.water_supply - 150.0) / 150.0
        } else if self.water_supply < 20.0 {
            (20.0 - self.water_supply) / 20.0
        } else {
            0.0
        };

        let silo_total: f64 = self.silo.values().sum::<u32>() as f64;
        let silo_err = if silo_total > 200.0 {
            (silo_total - 200.0) / 200.0 * 0.1 // mild penalty for overstocking
        } else {
            0.0
        };

        (soil_err + water_err + silo_err) / 3.0
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Soil tests ---

    #[test]
    fn soil_default_fertility() {
        let soil = Soil::new();
        let f = soil.fertility();
        assert!((50.0..85.0).contains(&f), "default fertility should be moderate: got {f}");
    }

    #[test]
    fn soil_deplete_reduces_values() {
        let mut soil = Soil::new();
        let before_n = soil.nitrogen;
        soil.deplete(&Soil { nitrogen: 10.0, phosphorus: 5.0, potassium: 5.0, moisture: 0.0, ph: 0.0, organic_matter: 0.0 });
        assert!(soil.nitrogen < before_n);
        assert!(soil.phosphorus < 50.0);
    }

    #[test]
    fn soil_deplete_clamps_at_zero() {
        let mut soil = Soil { nitrogen: 2.0, phosphorus: 0.0, potassium: 0.0, moisture: 0.0, ph: 6.5, organic_matter: 0.0 };
        soil.deplete(&Soil { nitrogen: 100.0, phosphorus: 0.0, potassium: 0.0, moisture: 0.0, ph: 0.0, organic_matter: 0.0 });
        assert_eq!(soil.nitrogen, 0.0);
    }

    #[test]
    fn soil_replenish_adds_values() {
        let mut soil = Soil { nitrogen: 10.0, phosphorus: 10.0, potassium: 10.0, moisture: 10.0, ph: 6.5, organic_matter: 10.0 };
        soil.replenish(&Soil { nitrogen: 20.0, phosphorus: 20.0, potassium: 20.0, moisture: 20.0, ph: 0.0, organic_matter: 20.0 });
        assert_eq!(soil.nitrogen, 30.0);
    }

    #[test]
    fn soil_replenish_clamps_at_100() {
        let mut soil = Soil { nitrogen: 95.0, phosphorus: 0.0, potassium: 0.0, moisture: 0.0, ph: 6.5, organic_matter: 0.0 };
        soil.replenish(&Soil { nitrogen: 20.0, phosphorus: 0.0, potassium: 0.0, moisture: 0.0, ph: 0.0, organic_matter: 0.0 });
        assert_eq!(soil.nitrogen, 100.0);
    }

    #[test]
    fn soil_tick_regen() {
        let mut soil = Soil { nitrogen: 40.0, phosphorus: 50.0, potassium: 50.0, moisture: 50.0, ph: 6.0, organic_matter: 20.0 };
        let before_n = soil.nitrogen;
        soil.tick();
        assert!(soil.nitrogen > before_n, "nitrogen should increase from fixation");
        assert!(soil.organic_matter < 20.0, "organic matter should decay");
    }

    #[test]
    fn soil_ph_drifts_toward_neutral() {
        let mut soil = Soil { nitrogen: 50.0, phosphorus: 50.0, potassium: 50.0, moisture: 50.0, ph: 5.0, organic_matter: 30.0 };
        soil.tick();
        assert!(soil.ph > 5.0, "pH should drift toward 7.0");
    }

    // --- CropType tests ---

    #[test]
    fn bean_fixes_nitrogen() {
        assert!(CropType::Bean.adds_nitrogen());
        assert!(CropType::Lavender.adds_nitrogen());
        assert!(!CropType::Wheat.adds_nitrogen());
    }

    #[test]
    fn crop_type_nutrient_need_positive() {
        for ct in [
            CropType::Wheat, CropType::Corn, CropType::Potato, CropType::Carrot,
            CropType::Tomato, CropType::Sunflower, CropType::Bean, CropType::Rice,
            CropType::Pumpkin, CropType::Lavender,
        ] {
            let need = ct.nutrient_need();
            assert!(need.nitrogen >= 0.0);
            assert!(need.phosphorus >= 0.0);
            assert!(need.potassium >= 0.0);
        }
    }

    #[test]
    fn crop_type_growth_time_range() {
        for ct in [
            CropType::Wheat, CropType::Corn, CropType::Potato, CropType::Carrot,
            CropType::Tomato, CropType::Sunflower, CropType::Bean, CropType::Rice,
            CropType::Pumpkin, CropType::Lavender,
        ] {
            let t = ct.growth_time();
            assert!((30..200).contains(&t), "growth time for {ct:?} = {t}");
        }
    }

    #[test]
    fn crop_type_yield_positive() {
        for ct in [
            CropType::Wheat, CropType::Corn, CropType::Potato, CropType::Carrot,
            CropType::Tomato, CropType::Sunflower, CropType::Bean, CropType::Rice,
            CropType::Pumpkin, CropType::Lavender,
        ] {
            assert!(ct.yield_amount() > 0);
        }
    }

    #[test]
    fn crop_type_water_need_range() {
        for ct in [
            CropType::Wheat, CropType::Corn, CropType::Potato, CropType::Carrot,
            CropType::Tomato, CropType::Sunflower, CropType::Bean, CropType::Rice,
            CropType::Pumpkin, CropType::Lavender,
        ] {
            let w = ct.water_need();
            assert!((0.0..=100.0).contains(&w), "water need for {ct:?} = {w}");
        }
    }

    #[test]
    fn rice_needs_most_water() {
        assert!(CropType::Rice.water_need() > CropType::Wheat.water_need());
        assert!(CropType::Rice.water_need() > CropType::Lavender.water_need());
    }

    // --- Crop tests ---

    #[test]
    fn crop_starts_not_mature() {
        let crop = Crop::new(CropType::Wheat, 0);
        assert!(!crop.is_mature());
        assert_eq!(crop.growth_stage, 0.0);
        assert_eq!(crop.health, 1.0);
    }

    #[test]
    fn crop_grows_with_good_conditions() {
        let mut crop = Crop::new(CropType::Carrot, 0);
        let soil = Soil::new();
        for _ in 0..60 {
            crop.tick(&soil, 50.0, 0.8);
        }
        assert!(crop.growth_stage > 0.0);
    }

    #[test]
    fn crop_matures_in_expected_time() {
        let mut crop = Crop::new(CropType::Carrot, 0);
        let soil = Soil::new();
        for _ in 0..200 {
            crop.tick(&soil, 50.0, 0.8);
        }
        assert!(crop.is_mature());
    }

    #[test]
    fn crop_harvest_returns_yield() {
        let mut crop = Crop::new(CropType::Wheat, 0);
        // Force maturity
        crop.growth_stage = 1.0;
        let y = crop.harvest();
        assert!(y > 0);
        assert_eq!(crop.growth_stage, 0.0);
    }

    #[test]
    fn crop_harvest_immature_returns_zero() {
        let mut crop = Crop::new(CropType::Wheat, 0);
        assert_eq!(crop.harvest(), 0);
    }

    #[test]
    fn crop_health_degrades_in_drought() {
        let mut crop = Crop::new(CropType::Tomato, 0);
        let soil = Soil { moisture: 5.0, ..Soil::new() };
        for _ in 0..100 {
            crop.tick(&soil, 5.0, 0.1);
        }
        assert!(crop.health < 1.0);
    }

    // --- FarmPlot tests ---

    #[test]
    fn farm_plot_new_has_no_crop() {
        let plot = FarmPlot::new(1.0);
        assert!(plot.crop.is_none());
    }

    #[test]
    fn farm_plot_tick_advances_soil() {
        let mut plot = FarmPlot::new(1.0);
        let before_n = plot.soil.nitrogen;
        plot.tick(&Weather::ideal(), 0);
        assert!(plot.soil.nitrogen != before_n || plot.soil.moisture != 50.0);
    }

    // --- Animal tests ---

    #[test]
    fn chicken_produces_eggs() {
        let product = Animal::Chicken.product();
        assert!(matches!(product, Some(FarmProduct::Egg(_))));
    }

    #[test]
    fn cow_produces_milk() {
        assert!(matches!(Animal::Cow.product(), Some(FarmProduct::Milk(_))));
    }

    #[test]
    fn sheep_produces_wool() {
        assert!(matches!(Animal::Sheep.product(), Some(FarmProduct::Wool(_))));
    }

    #[test]
    fn bee_produces_honey() {
        assert!(matches!(Animal::Bee.product(), Some(FarmProduct::Honey(_))));
    }

    #[test]
    fn pig_produces_manure() {
        assert!(matches!(Animal::Pig.product(), Some(FarmProduct::Manure(_))));
    }

    #[test]
    fn horse_produces_nothing() {
        assert!(Animal::Horse.product().is_none());
    }

    #[test]
    fn animal_feed_cost_positive() {
        for a in [Animal::Chicken, Animal::Cow, Animal::Sheep, Animal::Pig, Animal::Bee, Animal::Horse] {
            assert!(a.feed_cost() > 0.0, "feed cost for {a:?} should be positive");
        }
    }

    #[test]
    fn breeding_thresholds_in_range() {
        for a in [Animal::Chicken, Animal::Cow, Animal::Sheep, Animal::Pig, Animal::Bee, Animal::Horse] {
            let t = a.breeding_threshold();
            assert!((0.0..=1.0).contains(&t));
        }
    }

    // --- Farm tests ---

    #[test]
    fn farm_new_has_plots() {
        let farm = Farm::new(4);
        assert_eq!(farm.plots.len(), 4);
        assert!(farm.silo.is_empty());
        assert_eq!(farm.total_ticks, 0);
    }

    #[test]
    fn farm_plant_fills_empty_plot() {
        let mut farm = Farm::new(2);
        assert!(farm.plant(CropType::Wheat));
        assert!(farm.plots[0].crop.is_some());
        assert!(farm.plots[1].crop.is_none());
    }

    #[test]
    fn farm_plant_fails_when_full() {
        let mut farm = Farm::new(1);
        assert!(farm.plant(CropType::Wheat));
        assert!(!farm.plant(CropType::Corn));
    }

    #[test]
    fn farm_tick_increments() {
        let mut farm = Farm::new(2);
        let w = Weather::ideal();
        farm.tick(&w);
        assert_eq!(farm.total_ticks, 1);
        farm.tick(&w);
        assert_eq!(farm.total_ticks, 2);
    }

    #[test]
    fn farm_harvest_mature_collects() {
        let mut farm = Farm::new(1);
        farm.plant(CropType::Wheat);
        // Force maturity
        if let Some(ref mut c) = farm.plots[0].crop {
            c.growth_stage = 1.0;
        }
        let harvested = farm.harvest_mature();
        assert!(harvested > 0);
        assert!(farm.silo.get(&CropType::Wheat).copied().unwrap_or(0) > 0);
    }

    #[test]
    fn farm_harvest_immature_returns_zero() {
        let mut farm = Farm::new(1);
        farm.plant(CropType::Wheat);
        assert_eq!(farm.harvest_mature(), 0);
    }

    #[test]
    fn farm_add_compost_improves_soil() {
        let mut farm = Farm::new(1);
        let before = farm.plots[0].soil.nitrogen;
        farm.add_compost(10.0);
        assert!(farm.plots[0].soil.nitrogen > before);
    }

    #[test]
    fn farm_soil_health_average() {
        let farm = Farm::new(3);
        let h = farm.soil_health();
        assert!((0.0..=100.0).contains(&h));
    }

    #[test]
    fn farm_conservation_error_low_for_balanced() {
        let farm = Farm::new(3);
        let err = farm.conservation_error();
        assert!(err < 1.0, "conservation error for balanced farm should be low: {err}");
    }

    #[test]
    fn farm_rain_replenishes_water() {
        let mut farm = Farm::new(1);
        farm.water_supply = 50.0;
        farm.tick(&Weather { rainfall: 100.0, sunlight: 0.8, temperature: 22.0 });
        // Water should have increased from rain, minus plot consumption
        assert!(farm.water_supply > 45.0);
    }

    #[test]
    fn farm_livestock_consume_silo() {
        let mut farm = Farm::new(1);
        farm.silo.insert(CropType::Wheat, 100);
        farm.livestock.push(Animal::Cow);
        farm.tick(&Weather::ideal());
        assert!(farm.silo.get(&CropType::Wheat).copied().unwrap_or(0) < 100);
    }

    #[test]
    fn farm_bean_fixes_nitrogen_in_plot() {
        let mut farm = Farm::new(1);
        farm.plant(CropType::Bean);
        for _ in 0..10 {
            farm.tick(&Weather::ideal());
        }
        // Bean nitrogen fixation mechanism ran without panic
        assert!(farm.plots[0].soil.nitrogen >= 0.0);
    }

    #[test]
    fn farm_full_growth_cycle() {
        let mut farm = Farm::new(1);
        farm.plant(CropType::Carrot);
        for _ in 0..300 {
            farm.tick(&Weather::ideal());
        }
        let harvested = farm.harvest_mature();
        assert!(harvested > 0, "full cycle should produce harvest");
    }

    #[test]
    fn serde_roundtrip_farm() {
        let mut farm = Farm::new(2);
        farm.plant(CropType::Wheat);
        farm.livestock.push(Animal::Chicken);
        farm.silo.insert(CropType::Corn, 42);
        let json = serde_json::to_string(&farm).unwrap();
        let farm2: Farm = serde_json::from_str(&json).unwrap();
        assert_eq!(farm2.plots.len(), 2);
        assert_eq!(farm2.livestock.len(), 1);
        assert_eq!(farm2.silo.get(&CropType::Corn).copied(), Some(42));
    }

    #[test]
    fn serde_roundtrip_soil() {
        let soil = Soil::new();
        let json = serde_json::to_string(&soil).unwrap();
        let soil2: Soil = serde_json::from_str(&json).unwrap();
        assert!((soil2.nitrogen - soil.nitrogen).abs() < 0.001);
    }

    #[test]
    fn serde_roundtrip_crop_type() {
        for ct in [
            CropType::Wheat, CropType::Corn, CropType::Potato, CropType::Carrot,
            CropType::Tomato, CropType::Sunflower, CropType::Bean, CropType::Rice,
            CropType::Pumpkin, CropType::Lavender,
        ] {
            let json = serde_json::to_string(&ct).unwrap();
            let ct2: CropType = serde_json::from_str(&json).unwrap();
            assert_eq!(ct, ct2);
        }
    }
}
