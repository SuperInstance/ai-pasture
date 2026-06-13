# AI Pasture

**AI Pasture** is a Rust farming and agriculture simulation library implementing a NPK (nitrogen-phosphorus-potassium) soil model, crop growth dynamics, livestock management, and weather-driven ecosystem simulation for the Lau game universe.

## Why It Matters

Agricultural simulations require accurate biogeochemical modeling to produce believable game worlds. The NPK nutrient cycle, pH-dependent nutrient availability, and water-stress crop responses are well-documented in agronomy literature. This library encodes those models into a tick-based simulation where soil fertility is a weighted function of NPK balance, pH proximity to 6.75, moisture adequacy, and organic matter content. Crops consume nutrients proportional to species-specific needs, livestock convert silo grain into secondary products (eggs, milk, wool, manure), and legumes fix atmospheric nitrogen — all grounded in real agricultural science. This creates gameplay where sustainable farming practices (composting, crop rotation with legumes, water management) emerge as optimal strategies rather than being scripted.

## How It Works

**Soil fertility** is a weighted average across four factors:

```
fertility = 0.35 × NPK_avg + 0.15 × pH_score + 0.25 × moisture_score + 0.25 × OM_score
```

Where pH_score peaks at 6.75 with linear falloff, and moisture_score follows a trapezoidal function (ideal: 20–80%). This matches real soil science: NPK is the dominant factor, pH controls nutrient bioavailability, and organic matter improves both water retention and slow-release nutrients.

**Crop growth** per tick:

```
Δgrowth = (1/growth_time) × (0.4 × fertility + 0.3 × water_factor + 0.3 × sunlight)
```

Where `water_factor = min(1.0, soil_moisture / crop_water_need)`. Stress from drought or poor soil degrades health: `Δhealth = −0.01 × stress_factor`. At health = 0, the crop dies and adds organic matter to the soil.

**Nitrogen fixation:** Bean and lavender crops add +0.15 N per tick to soil, modeling Rhizobium symbiosis. This makes legumes valuable for crop rotation — a real farming strategy that the simulation rewards organically.

**Conservation error:** The farm computes a conservation deviation:

```
error = (soil_deviation + water_deviation + silo_deviation) / 3
```

This measures how far the farm is from sustainable equilibrium, directly connecting to the γ + η = C conservation framework.

## Quick Start

```rust
fn main() {
    let mut farm = Farm::new(4);
    farm.plant(CropType::Wheat);
    farm.livestock.push(Animal::Chicken);

    for _ in 0..300 {
        farm.tick(&Weather::ideal());
    }

    let harvested = farm.harvest_mature();
    println!("Harvested {} units", harvested);
    println!("Soil health: {:.1}", farm.soil_health());
}
```

## API

| Type | Description |
|------|-------------|
| `Soil` | NPK, moisture, pH, organic matter with fertility scoring |
| `CropType` | 10 crop species with per-species nutrient needs |
| `Crop` | Growth stage (0.0–1.0) and health tracking |
| `FarmPlot` | Soil + optional crop with tick simulation |
| `Animal` | 6 species with products and feed costs |
| `FarmProduct` | Egg, Milk, Wool, Honey, Manure |
| `Weather` | Rainfall, sunlight, temperature |
| `Farm` | Top-level: plots, livestock, silo, water, compost |

## Architecture Notes

AI Pasture models the **ecological substrate** of the Lau game world. Within γ + η = C, the farm's conservation error function directly instantiates the conservation law: sustainable farms (low error) represent γ + η equilibrium, while degraded farms (high error) represent conservation violations. The species coexistence dynamics mirror Law 3 (100% ecological resilience).

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Brady, N.C. & Weil, R.R. (2008). *The Nature and Properties of Soils*. 14th ed. Pearson.
2. Hoogenboom, G. et al. (2004). "Decision Support System for Agrotechnology Transfer." *DSSAT v4*.

## License

MIT
