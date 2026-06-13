# ai-pasture

**AI Pasture** is a Rust farming and agriculture simulation library implementing a NPK (nitrogen-phosphorus-potassium) soil nutrient model, crop growth dynamics, livestock management, and weather-driven ecosystem simulation for the Lau game universe. It models a tick-based ecosystem where soil fertility, crop health, water cycles, and animal husbandry interact through biogeochemically-grounded equations.

## Why It Matters

Agricultural simulations require accurate biogeochemical modeling to produce believable game worlds. The NPK nutrient cycle, pH-dependent nutrient availability, and water-stress crop responses are well-documented in agronomy literature. This library encodes those models into a tick-based simulation where soil fertility is a weighted function of NPK balance, pH proximity to 6.75, moisture adequacy, and organic matter content.

Crops consume nutrients proportional to species-specific needs. Livestock convert silo grain into secondary products (eggs, milk, wool, manure). Legumes fix atmospheric nitrogen via Rhizobium symbiosis. Weather drives soil moisture through rainfall. All of these create gameplay where sustainable farming practices — composting, crop rotation with legumes, water management — emerge as optimal strategies rather than being scripted.

This matters because it means the game's economy is grounded in real science. Players who understand crop rotation, nitrogen fixation, and soil conservation have a genuine advantage. The simulation rewards ecological literacy.

## How It Works

### Soil Fertility Model

**Soil fertility** is a weighted average across four factors:

```
fertility = 0.35 × NPK_avg + 0.15 × pH_score + 0.25 × moisture_score + 0.25 × OM_score
```

Where:

- **NPK_avg** = (N + P + K) / 3, each ∈ [0, 100]
- **pH_score** = 1.0 − |pH − 6.75| / 6.75 (peaks at 6.75, linear falloff)
- **moisture_score** = trapezoidal function (ideal: 20–80%)
- **OM_score** = organic_matter / 100

This matches real soil science: NPK is the dominant factor (35% weight), pH controls nutrient bioavailability (15%), moisture enables nutrient transport (25%), and organic matter improves both water retention and slow-release nutrients (25%).

**Complexity:** Fertility computation is O(1) — constant arithmetic per plot per tick.

### Crop Growth Dynamics

**Crop growth** per tick follows:

```
Δgrowth = (1 / growth_time) × (0.4 × fertility + 0.3 × water_factor + 0.3 × sunlight)
```

Where `water_factor = min(1.0, soil_moisture / crop_water_need)`.

Each crop species has specific parameters:

| Crop | Growth Time (ticks) | Water Need | Nitrogen Fixer | Yield |
|------|-------------------|------------|----------------|-------|
| Wheat | 100 | 40 | No | 8 |
| Corn | 120 | 60 | No | 12 |
| Rice | 90 | 90 | No | 10 |
| Bean | 80 | 35 | **Yes** | 6 |
| Lavender | 60 | 20 | **Yes** | 4 |
| Potato | 70 | 45 | No | 14 |

Growth accumulates from 0.0 to 1.0 (mature). At maturity, `harvest()` returns the yield amount and resets the crop.

**Health degradation** under stress:

```
Δhealth = −0.01 × stress_factor
```

Where stress_factor accumulates from drought (moisture < water_need) and poor soil (fertility < 0.3). At health = 0, the crop dies and returns organic matter to the soil — modeling decomposition.

### Nitrogen Fixation

Bean and lavender crops add +0.15 N per tick to soil:

```
if crop.adds_nitrogen:
    soil.N = min(100, soil.N + 0.15)
```

This models **Rhizobium symbiosis**: legumes host nitrogen-fixing bacteria in root nodules that convert atmospheric N₂ to plant-available NH₃ (ammonia). This makes legumes valuable for crop rotation — a real farming strategy that the simulation rewards organically.

### Conservation Error

The farm computes a conservation deviation:

```
error = (soil_deviation + water_deviation + silo_deviation) / 3
```

Where:

- **soil_deviation** = average |fertility − 50| / 50 across plots (deviation from ideal mid-range)
- **water_deviation** = penalty if water < 20 or > 150 (drought or flood)
- **silo_deviation** = mild penalty for overstocking (> 200 units)

This directly connects to the γ + η = C conservation framework: sustainable farms (low error) represent γ + η equilibrium.

### Livestock Economics

Each animal species has a feed cost and produces a product:

| Animal | Feed Cost | Product | Breeding Threshold |
|--------|----------|---------|-------------------|
| Chicken | 1.0 | Eggs (1) | 0.50 |
| Cow | 4.0 | Milk (4) | 0.70 |
| Sheep | 2.5 | Wool (2) | 0.60 |
| Pig | 3.0 | Manure (1.5) | 0.55 |
| Bee | 0.5 | Honey (1) | 0.40 |
| Horse | 3.5 | — | 0.75 |

Livestock consume from the silo each tick. If feed is insufficient, animals don't produce. Pigs generate manure that adds to compost, closing the nutrient loop.

**Big-O summary:**

| Operation | Time | Space |
|-----------|------|-------|
| Farm tick | O(P + L) where P = plots, L = livestock | O(P + L) |
| Harvest | O(P) | O(1) |
| Plant | O(P) | O(1) |
| Conservation error | O(P) | O(1) |
| Serialization | O(P + L + S) where S = silo entries | O(P + L + S) |

## Quick Start

```rust
use ai_pasture::*;

// Create a 4-plot farm
let mut farm = Farm::new(4);

// Plant crops
farm.plant(CropType::Wheat);
farm.plant(CropType::Bean); // nitrogen fixer

// Add livestock
farm.livestock.push(Animal::Chicken);
farm.livestock.push(Animal::Cow);

// Simulate 300 ticks of ideal weather
for _ in 0..300 {
    farm.tick(&Weather::ideal());
}

// Harvest mature crops
let yield_amount = farm.harvest_mature();
println!("Harvested {} units", yield_amount);
println!("Soil health: {:.1}", farm.soil_health());
println!("Conservation error: {:.4}", farm.conservation_error());

// Serialize/deserialize (for save games)
let json = serde_json::to_string(&farm).unwrap();
let restored: Farm = serde_json::from_str(&json).unwrap();
```

## API

| Type | Key Methods | Description |
|------|-------------|-------------|
| `Soil` | `new`, `fertility`, `deplete`, `replenish`, `tick` | NPK + moisture + pH + organic matter |
| `CropType` | `nutrient_need`, `growth_time`, `water_need`, `yield_amount`, `adds_nitrogen` | 10 crop species with per-species parameters |
| `Crop` | `new`, `tick`, `is_mature`, `harvest` | Growth stage (0.0–1.0) and health |
| `FarmPlot` | `new`, `tick`, `plant` | Soil + optional crop |
| `Animal` | `product`, `feed_cost`, `breeding_threshold` | 6 livestock species |
| `FarmProduct` | `Egg`, `Milk`, `Wool`, `Honey`, `Manure` | Enumerated outputs |
| `Weather` | `ideal`, custom fields | Rainfall, sunlight, temperature |
| `Farm` | `new`, `tick`, `plant`, `harvest_mature`, `soil_health`, `conservation_error` | Top-level simulation |

All types derive `Serialize`/`Deserialize` for save-game persistence.

## Architecture Notes

AI Pasture models the **ecological substrate** of the Lau game world. Within γ + η = C, the farm's conservation error function directly instantiates the conservation law: sustainable farms (low error) represent γ + η equilibrium, while degraded farms (high error) represent conservation violations. The species coexistence dynamics mirror Law 3 (100% ecological resilience).

The soil nutrient cycle is a closed loop: crops deplete soil (γ), weather and compost replenish it (η), and the total fertility budget is conserved (C). Nitrogen fixation by legumes adds a small amount of "new" nitrogen, modeling biological nitrogen fixation from the atmosphere — the one external input that keeps the cycle from being perfectly closed.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Brady, N.C. & Weil, R.R. (2008). *The Nature and Properties of Soils*. 14th ed. Pearson. (Standard soil science reference)
2. Hoogenboom, G. et al. (2004). "Decision Support System for Agrotechnology Transfer." *DSSAT v4*. (Crop simulation models)
3. Giller, K.E. (2001). *Nitrogen Fixation in Tropical Cropping Systems*. 2nd ed. CABI. (Biological N fixation)
4. Ritchie, J.T. (1998). "Soil water balance and plant water stress." *Systems Approaches for Sustainable Agricultural Development*, 41–54.

## License

MIT
