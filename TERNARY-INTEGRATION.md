# Ternary Integration: Every Crate as a Game Mechanic

**Status:** Architecture Document · **Date:** 2026-06-04 · **Author:** Synthesis Agent

> AI-Pasture isn't a game with some math bolted on. It's real ecology, real genetics, real conservation — wrapped in play. Every ternary crate is a game system, and every game system teaches something true about how the world works.

---

## Table of Contents

1. [The Design Philosophy](#1-philosophy)
2. [Crate-to-Game-System Map](#2-crate-map)
3. [The Spreadsheet-as-Farm-Dashboard](#3-spreadsheet)
4. [SMP Seeds as NPC Advisors](#4-smp-npc)
5. [Minecraft → Real Physics Pipeline](#5-minecraft)
6. [The Education Layer](#6-education)
7. [Multiplayer: Competitive Farming](#7-multiplayer)
8. [Implementation Roadmap](#8-roadmap)

---

## 1. The Design Philosophy

Most educational games fail because they're educational first and games second. The kid can smell the worksheet behind the cartoon mascot. AI-Pasture inverts this: it's a game first, and the education is inseparable from the mechanics because the mechanics *are* real science.

The ternary fleet gives us something no other educational game has: a complete computational ecosystem where conservation laws are actual physics, evolution is actual genetics, and ecosystem dynamics are actual population biology. We don't simulate ecology for the game — we run real ecology and the game is the interface.

Every ternary crate maps to a game system. Not a simplified version of the crate — the actual crate. Kids don't play with toy conservation; they play with `conservation-verify`. They don't see a cartoon of evolution; they run `ternary-evolution`. The game is the science, and the science is the game.

---

## 2. Crate-to-Game-System Map

### Core Game Systems

| Ternary Crate | Game System | What Kids Experience |
|---|---|---|
| `ternary-cell` | **Every living thing on the farm** | Each crop, animal, bug, and microbe is a ternary cell running a tick cycle. The cell predicts what will happen next, perceives the environment, gets surprised by unexpected events, adjusts its behavior, recycles old strategies, and obeys conservation laws. Kids see: plants that "think" — they grow toward light, avoid drought, compete for nutrients. |
| `ternary-ecosystem` | **The food web** | Five species (Explorer, Diplomat, Marksman, Climber, Prospector) become pollinators, pests, predators, crops, and decomposers. Kids see: bees pollinating flowers, ladybugs eating aphids, foxes hunting rabbits. The ecosystem crate runs the population dynamics — when rabbits boom, foxes thrive, then rabbits crash, then foxes starve. Real Lotka-Volterra cycles. |
| `ternary-evolution` / `evolution-ternary` | **Crop and animal breeding** | Players breed crops and animals with real genetics. Parent traits (drought resistance, growth rate, yield, flavor) recombine and mutate according to ternary crossover and mutation operators. Kids see: cross-pollinating two wheat varieties to get one that grows faster in dry soil. This is what farmers have done for 10,000 years — now kids do it with real genetic algorithms. |
| `ternary-dice` | **Weather and random events** | Stochastic events powered by different distribution "flavors." Weather rolls on `3d6` (bell curve — mostly mild, rare extremes). Market prices on `1d100` (uniform — unpredictable). Pest outbreaks on power-law (usually small, occasionally devastating). Kids see: "It's going to rain tomorrow" — but the forecast has a probability distribution, not a certainty. |
| `conservation-verify` / `conservation-matrix-rs` | **The physics of the farm** | Resources are conserved. Water used by crops reduces the reservoir. Nutrients removed by harvesting must be replaced by fertilization or rotation. Energy entering the system (sunlight) equals energy leaving (harvest + waste + heat). The conservation checker runs every tick. When it catches a violation (player used more water than they have), the system compensates — crops wilt. Kids see: the conservation gauge on their dashboard. Green means balanced. Red means trouble. |
| `ternary-fitness` | **Farm health scoring** | Every crop, animal, and ecosystem element has a fitness score. Fitness depends on genetics, environment, competition, and resource availability. Kids see: health bars that reflect real biological fitness, not arbitrary game design. A crop in poor soil has low fitness. Move it to good soil — fitness rises. |
| `ternary-rigging` | **The "what-if" machine** | Grab any farm parameter (rainfall, temperature, soil pH) and shake it. Watch the ecosystem ripple in real time. Kids see: dragging the "rainfall" slider left — crops start yellowing, pests increase (stressed plants are vulnerable), the food web reshuffles. Dragging it right — crops green up, but so do weeds. The rigging system makes every connection visible. |
| `ternary-arena` | **Competitive farming** | Players' farms compete. Each farm is an ecosystem; the arena runs tournaments to see which farm produces more food, uses fewer resources, and supports more biodiversity. Kids see: "Your farm vs. Alex's farm — who can grow more wheat on the same land?" Real competition, real science. |

### Supporting Game Systems

| Ternary Crate | Game System | What Kids Experience |
|---|---|---|
| `ternary-weather` | **Seasonal cycles** | Spring planting, summer growing, fall harvest, winter rest. Each season affects growth rates, water availability, pest populations, and market prices. Kids learn that timing matters — planting too early or too late has real consequences. |
| `ternary-energy` | **Energy budget** | Every action costs energy. Plowing costs energy. Planting costs energy. Harvesting costs energy. Energy comes from food (your harvest) and sunlight (passive). Kids learn that farming is an energy budget — you can't do more than your energy allows. |
| `ternary-entropy` | **Diversity metrics** | The entropy of your farm measures how diverse it is. High entropy = many different crops and animals. Low entropy = monoculture. Kids see: "Your farm entropy is 0.3 — that's low. Consider adding a new crop." They learn that diversity is a measurable, quantifiable thing. |
| `ternary-graph` | **Connection map** | Visualize every connection on the farm: which crops depend on which pollinators, which pests threaten which crops, which predators eat which pests. Kids see a web of life — not a textbook diagram, but their actual farm's actual dependencies. |
| `ternary-sensor` / `ternary-kalman` | **Farm sensors** | Soil moisture sensors, weather stations, pest traps. Each sensor gives noisy readings that the Kalman filter cleans up. Kids see: "Your soil moisture sensor says 45%, but the Kalman estimate (accounting for sensor noise) is 42%." They learn that real measurements have uncertainty. |
| `ternary-bayesian` | **Forecasting** | Bayesian updating for weather predictions, pest outbreak probabilities, market price forecasts. Each prediction updates as new data arrives. Kids see: "70% chance of rain tomorrow" — and when it doesn't rain, the forecast adjusts for next time. |
| `ternary-markov` | **State transitions** | Crops progress through growth stages (seed → sprout → vegetative → flowering → fruiting → harvest). Each transition has a probability that depends on conditions. Kids see: "Your tomato is 60% likely to reach flowering this week given current conditions." |
| `ternary-noise` | **Random mutations** | Genetic mutations during breeding are stochastic. Most are neutral, some are harmful, a few are beneficial. Kids see: "Your new wheat variety has a mutation — drought resistance +2." They learn that mutation is random, but selection is not. |
| `ternary-time` / `ternary-rhythm` | **Circadian and seasonal rhythms** | Plant growth follows diurnal cycles. Animals have activity patterns. Seasons follow their rhythms. The game world has a clock that drives all of these. Kids learn that biology is rhythmic. |
| `ternary-attention` | **Agent focus** | Each farm organism has limited attention — it can't perceive everything. Attention is allocated based on fitness and surprise. Kids see: "Your bee colony is focused on the clover field today — they noticed something new there." |
| `ternary-dynamics` / `ternary-chaos` | **Emergent dynamics** | The farm is a dynamical system. Small changes can cascade (chaos) or stabilize (attractors). Kids experience both: a butterfly effect (one extra rabbit → population explosion) and a stabilizer (crop rotation prevents soil exhaustion). |
| `ternary-games` / `ternary-game-theory` | **Market economics** | Crop prices respond to supply and demand. If everyone grows wheat, wheat prices crash. Game theory: the best crop depends on what other players grow. Kids see: "Wheat is $5/bushel — but 8 players grew wheat this season. Next season, it'll be $2." |
| `ternary-market` | **Trading and economy** | Buy and sell crops, animals, tools, and land. Prices are set by supply, demand, and random events. Kids learn basic economics through farming. |
| `ternary-econ` | **Resource economics** | Limited resources (land, water, labor) must be allocated efficiently. Opportunity cost is real — every acre of wheat is an acre not growing corn. Kids learn that economics is about trade-offs. |
| `ternary-visualization` / `ternary-visualizer` | **Farm rendering** | The farm is rendered as a living landscape. Not pixel art — a real visualization of ternary cell states. Healthy crops are green and vibrant. Stressed crops are yellow and droopy. Dead crops are brown. The visualization is the data. |
| `ternary-spreadsheet` | **The farm dashboard** | The spreadsheet IS the farm dashboard. Rows are crops, animals, and resources. Columns are time periods, properties, and metrics. Formulas compute yields, costs, and profits. The spreadsheet is the control center. |

### Deep Integration Systems

| Ternary Crate | Game System | What Kids Experience |
|---|---|---|
| `ternary-prophet` | **Long-term forecasting** | The prophet crate predicts farm outcomes weeks and months ahead based on current trends. Kids see: "At this rate, your soil nitrogen will be depleted in 3 seasons." Forward planning becomes a game mechanic. |
| `ternary-curriculum` | **Progressive difficulty** | The curriculum crate adjusts difficulty based on player skill. New players get simple farms with forgiving ecosystems. Experienced players face complex multi-biome farms with tight resource constraints. The game grows with the kid. |
| `ternary-pipeline` | **Processing chains** | Wheat → flour → bread. Milk → cheese. Grapes → wine (or grape juice). Each step in the processing chain takes time, resources, and energy. Kids learn about supply chains and value addition. |
| `ternary-logic` | **Farm automation** | "If soil moisture < 30%, turn on irrigation." "If pest count > 5 per plant, release ladybugs." Players build logic circuits for farm automation using ternary logic gates. It's redstone for real farming. |
| `ternary-circuit` | **Farm infrastructure** | Irrigation systems, drainage, fences, paths, storage. Each infrastructure element is a circuit of connected components. Kids learn systems thinking by building farm infrastructure. |
| `ternary-scheduling` | **Farm calendar** | Plan planting, tending, and harvesting schedules. Optimize for yield, resource use, and labor. Kids learn project management through farming. |
| `ternary-planning` | **Strategic planning** | Set long-term goals (5-year farm plan) and watch the plan play out. Adjust as conditions change. Kids learn that planning matters, but adaptation matters more. |
| `ternary-transfer` | **Knowledge transfer** | Strategies that work on one farm can be transferred to another. The transfer crate enables sharing of crop varieties, farming techniques, and ecosystem configurations between players. |
| `ternary-reef` | **Biodiversity hotspots** | Some areas of the farm are biodiversity hotspots (like coral reefs). Protecting them provides ecosystem services (pollination, pest control, soil health). The reef crate identifies and manages these critical areas. |
| `ternary-explain` | **The "why" system** | When something happens on the farm (crop failure, pest outbreak, market crash), the explain crate provides an explanation in kid-friendly language. "Your wheat failed because soil nitrogen was too low. You planted wheat three seasons in a row, and wheat removes nitrogen from soil. Next time, try planting beans first — they add nitrogen." |
| `ternary-trees` | **Decision trees** | Farm decisions are visualized as decision trees. "If I plant wheat here, and it rains, yield is X. If it doesn't rain, yield is Y." Kids learn decision analysis through branching scenarios. |
| `ternary-science` | **The science journal** | A running log of discoveries and observations. "Day 45: Noticed that rotating beans and wheat improved wheat yield by 30%. Hypothesis: beans add nitrogen to soil. Experiment: plant wheat after beans vs. after wheat. Result: confirmed." Kids learn the scientific method by doing it. |

---

## 3. The Spreadsheet-as-Farm-Dashboard

### The UX

The farm dashboard is a living spreadsheet — every cell is a ternary agent, every formula is a real computation, every value is real data:

```
┌─────────────────────────────────────────────────────────────────────┐
│  🌾 AI-Pasture Farm Dashboard — Spring, Year 2                      │
├──────────┬──────────┬──────────┬──────────┬──────────┬─────────────┤
│ Crop     │ Acres    │ Health   │ Yield Est│ Revenue  │ Notes       │
├──────────┼──────────┼──────────┼──────────┼──────────┼─────────────┤
│ Wheat    │ 12       │ =FIT(A2) │ =EST(A2) │ =REV(A2) │ Rotation yr2│
│ Corn     │ 8        │ =FIT(B2) │ =EST(B2) │ =REV(B2) │ New variety │
│ Beans    │ 6        │ =FIT(C2) │ =EST(C2) │ =REV(C2) │ N-fixer ✓   │
│ Tomatoes │ 4        │ =FIT(D2) │ =EST(D2) │ =REV(D2) │ Greenhouse  │
├──────────┼──────────┼──────────┼──────────┼──────────┼─────────────┤
│ TOTAL    │ =SUM(AC) │ =AVG(HT) │          │ =SUM(REV)│             │
├──────────┴──────────┴──────────┴──────────┴──────────┴─────────────┤
│                                                                     │
│  =CONSERVATION()  →  γ+H = 1.31 (target: 1.28) ✅ SLIGHTLY HIGH    │
│  =ENTROPY()       →  Farm diversity: 0.67 (moderate)                │
│  =FORECAST(7d)    →  70% rain Tue, 20% frost risk Thu              │
│  =EVOLVE(wheat,10)→  Breeding new variety (gen 10 in progress...)  │
│  =ROLL("3d6")     →  Weather event: 11 (moderate rain) 🌧️          │
│                                                                     │
│  📊 Conservation Gauge: ████████░░ 82% balanced                     │
│  🐝 Pollinator Health:  ███████░░░ 71%                              │
│  🌱 Soil Nitrogen:      ████░░░░░░ 40% (LOW — consider beans)      │
│  💧 Water Reservoir:    █████████░ 92%                               │
└─────────────────────────────────────────────────────────────────────┘
```

### Custom Formula Functions

The spreadsheet provides domain-specific formula functions that map directly to ternary crates:

| Formula | Crate | What It Does |
|---|---|---|
| `=FIT(crop)` | `ternary-fitness` | Compute fitness score for a crop/animal |
| `=EST(crop)` | `ternary-markov` + `ternary-kalman` | Estimate yield based on current conditions and historical data |
| `=REV(crop)` | `ternary-market` | Project revenue based on current market prices and estimated yield |
| `=CONSERVATION()` | `conservation-verify` | Check farm-wide conservation status |
| `=ENTROPY()` | `ternary-entropy` | Measure farm biodiversity/diversity |
| `=FORECAST(period)` | `ternary-bayesian` + `ternary-weather` | Bayesian weather forecast for the period |
| `=EVOLVE(crop, generations)` | `ternary-evolution` | Breed a new crop variety over N generations |
| `=ROLL(dice_spec)` | `ternary-dice` | Roll dice for stochastic events |
| `=PREDICT(crop, days)` | `ternary-prophet` | Long-term crop outcome prediction |
| `=EXPLAIN(event)` | `ternary-explain` | Explain why something happened |
| `=GRAPH(entity)` | `ternary-graph` | Show dependency graph for an entity |
| `=SHAKE(param, range)` | `ternary-rigging` | Oscillate a parameter and watch ripples |
| `=ARENA(my_farm, opponent)` | `ternary-arena` | Run a farming competition |
| `=SENSE(sensor_id)` | `ternary-sensor` | Read from a farm sensor |
| `=AUTOMATE(rule)` | `ternary-logic` | Create an automation rule |

### The Farm as Spreadsheet

The entire farm state lives in the spreadsheet. This isn't a separate UI that talks to a simulation — the spreadsheet IS the simulation. When a kid changes a value in the spreadsheet (e.g., planting 5 more acres of wheat), the change propagates through the ternary cell grid exactly like a spreadsheet recalculation:

1. Cell B2 changes from 12 to 17 (5 more acres of wheat).
2. `=FIT(A2)` recalculates — fitness drops because the soil can't support that much wheat.
3. `=CONSERVATION()` recalculates — conservation drifts because more wheat means more water and nutrient consumption.
4. The conservation gauge updates — it tilts toward red.
5. `=EXPLAIN(drop)` fires — "Wheat fitness dropped because soil nitrogen can't support 17 acres. You'd need 20% more nitrogen fertilizer or 4 fewer acres."

The kid learns cause and effect by editing a spreadsheet. The same way adults learn cause and effect in financial models — but this model is a farm, and the physics is real.

---

## 4. SMP Seeds as NPC Advisors

### The NPC System

Every farm has advisors — NPCs who help the player make decisions. Each NPC is an SMP seed applied to the same base model:

| NPC Advisor | Seed Profile | Personality | Advice Style |
|---|---|---|---|
| **Old Farmer Jeb** | Conservative, experience-weighted | Cautious, speaks in proverbs | "Don't plant corn before the last frost. My grandpappy learned that the hard way." |
| **Dr. Chen** | Analytical, data-driven | Precise, uses numbers | "Your soil nitrogen is 42 ppm. Optimal range for wheat is 60-80. I recommend applying 2.3 kg/acre of nitrogen fertilizer." |
| **Luna the Explorer** | Creative, experimental | Enthusiastic, loves trying new things | "What if we tried companion planting? Marigolds next to tomatoes — I heard it keeps pests away!" |
| **Marcus the Strategist** | Competitive, game-theoretic | Strategic, thinks about markets | "Everyone's planting wheat this season. Corn will be scarce. If you grow corn, you'll get premium prices." |
| **Bee Guardian** | Ecological, holistic | Gentle, thinks in systems | "Your pollinator population is declining. Consider planting wildflowers along the field edges — the bees need habitat, not just crops." |

### How NPCs Work

Each NPC runs as an SMP seed applied to the same base LLM:

1. **Input:** The current farm state (spreadsheet values, sensor readings, forecasts).
2. **Seed application:** The NPC's seed shapes the model's disposition — Old Farmer Jeb's seed promotes conservative advice; Luna's seed promotes experimental advice.
3. **Output:** The NPC gives advice in character, referencing real farm data.

The base model is the same for all NPCs. The seed alone creates the personality difference. This means:
- All NPCs have access to the same farm data and knowledge.
- The difference is in *how* they reason about it, not *what* they know.
- Players learn that different perspectives on the same data lead to different strategies.
- The NPCs sometimes disagree — and that's a feature, not a bug.

### NPC Evolution

NPCs aren't static. As the player progresses:

1. The NPC's seed adapts based on the player's farming style. If the player is aggressive, Old Farmer Jeb becomes more cautious in response.
2. NPCs learn from the player's successes and failures. If the player ignores Dr. Chen's nitrogen advice and the crop fails, Dr. Chen references it next time.
3. New NPCs unlock as the farm grows. A large farm unlocks a Logistics Advisor. A diverse farm unlocks a Biodiversity Specialist.
4. The player can breed new NPCs by "crossing" existing ones — combining Dr. Chen's analytical seed with Luna's experimental seed creates a Data-Driven Innovator.

---

## 5. Minecraft → Real Physics Pipeline

### The Bridge

Kids who've played Minecraft already understand farming mechanics. AI-Pasture meets them where they are and takes them further:

| Minecraft Concept | AI-Pasture Equivalent | What Changes |
|---|---|---|
| **Bonemeal = instant growth** | Nitrogen fertilizer with real cycling | Bonemeal is magic. Nitrogen fertilizer is chemistry. The crop needs nitrogen to grow, and it gets it from fertilizer, compost, or nitrogen-fixing bacteria in bean roots. Using too much fertilizer causes nitrogen runoff (pollution). Kids learn: there are no free lunches. |
| **Breeding = feed two animals → baby** | Population genetics with real inheritance | Minecraft breeding is deterministic — same parents, same baby (random from a small pool). AI-Pasture breeding uses real Mendelian genetics with crossover and mutation. Parents contribute alleles. Some traits are dominant, some recessive. Kids learn: inheritance is probabilistic, not deterministic. |
| **Light levels = growth speed** | Photosynthesis model | Minecraft's light levels are a binary switch (enough light → grow, not enough → don't). AI-Pasture models real photosynthesis: growth rate is proportional to light intensity, CO₂ concentration, and water availability. Kids learn: biological processes are continuous, not binary. |
| **Biomes = fixed terrain** | Dynamic ecosystems | Minecraft biomes are static — a desert is always a desert. AI-Pasture ecosystems are dynamic — overgrazing turns grassland into desert, reforestation turns desert back into grassland. Kids learn: ecosystems can change, and human actions drive the change. |
| **Redstone = logic circuits** | Ternary logic automation | Minecraft redstone is binary (on/off). AI-Pasture's automation uses ternary logic (promote/silence/suppress). This is more expressive — a ternary irrigation valve can be "full/open/closed" instead of just "on/off." Kids learn: the world isn't binary. |
| **Mob farms = population dynamics** | Lotka-Volterra predator-prey | Minecraft mob farms exploit spawn mechanics. AI-Pasture's "pest management" requires understanding predator-prey dynamics. Release too many ladybugs → they eat all the aphids → ladybugs starve → aphids rebound worse. Kids learn: ecosystems have carrying capacities and overshoot dynamics. |
| **Crop growth ticks** | Real growth models | Minecraft crops grow on random ticks. AI-Pasture crops grow according to real growth models: logistic curves, degree-day accumulation, and resource-dependent rates. Kids learn: growth isn't random — it follows predictable patterns that depend on conditions. |

### The Transition

The game doesn't start with a lecture about conservation. It starts with Minecraft-like mechanics that feel familiar:

**Level 1 (Tutorial):** Plant wheat. It grows. Harvest. Plant again. Looks like Minecraft. The kid is comfortable.

**Level 2 (Simple constraints):** The kid notices yields are dropping. Why? Soil is tired. The game introduces compost (Minecraft-like bonemeal). Yields recover. But now there's a new concept: soil health.

**Level 3 (Real physics begins):** The kid tries to plant 50 acres of wheat on a small farm. The conservation gauge turns red. Crops start dying. The game explains: "You're using more water than your land has. Try planting less, or add irrigation." The kid encounters conservation for the first time — as a game constraint, not a lesson.

**Level 4 (Ecosystems):** Pests appear. The kid can spray pesticide (expensive, harms pollinators) or introduce ladybugs (cheaper, but takes time). The ecosystem crate runs the predator-prey dynamics. The kid learns: there are always trade-offs.

**Level 5 (Genetics):** The kid discovers breeding. Cross a drought-resistant wheat with a high-yield wheat. Sometimes the offspring gets both traits. Sometimes it gets neither. The evolution crate runs real genetic algorithms. The kid learns: inheritance is probabilistic.

**Level 6 (Markets):** Multiple players are farming. Market prices respond to supply and demand. The kid learns: the best crop depends on what everyone else is growing. Game theory, through farming.

**Level 7 (The full system):** Weather, markets, genetics, ecosystems, conservation — everything interacts. The kid is managing a complex system. They're using the spreadsheet to track everything. They're shaking parameters to see what happens. They're competing in arenas. They're receiving advice from NPCs.

They've gone from Minecraft to systems ecology, and they did it through play.

---

## 6. The Education Layer

### What Kids Learn Without Knowing They're Learning It

| Game Activity | What They're Actually Doing | Academic Subject |
|---|---|---|
| Planting crops in rows | Spatial optimization, resource allocation | Geometry, optimization |
| Breeding better varieties | Genetic algorithms, inheritance | Biology, genetics |
| Managing water resources | Conservation of mass, budgeting | Physics, economics |
| Responding to weather events | Probabilistic reasoning, risk management | Statistics, decision theory |
| Competing in farm arenas | Game theory, strategic thinking | Economics, mathematics |
| Reading the conservation gauge | Thermodynamics, invariant preservation | Physics, conservation laws |
| Using the spreadsheet dashboard | Data analysis, formula writing | Mathematics, computer science |
| Building automation rules | Logic programming, conditional reasoning | Computer science, logic |
| Observing predator-prey dynamics | Population dynamics, differential equations | Biology, mathematics |
| Managing soil nutrients | Chemical cycling, stoichiometry | Chemistry, ecology |
| Planning crop rotations | Cyclic groups, optimization | Mathematics, agriculture |
| Interpreting forecasts | Bayesian reasoning, uncertainty quantification | Statistics, meteorology |

### The Hidden Curriculum

The game teaches meta-skills that no textbook covers:

1. **Systems thinking:** Everything is connected. Changing one thing affects everything else. The conservation law enforces this — you can't change one variable without the system compensating.

2. **Trade-offs:** There are no optimal solutions, only Pareto-optimal ones. You can't maximize yield AND sustainability AND profit simultaneously. You choose your priorities.

3. **Uncertainty:** The future is probabilistic, not deterministic. Forecasts have error bars. Decisions must be made under uncertainty.

4. **Emergence:** Complex behavior arises from simple rules. The ecosystem's behavior emerges from individual organisms following their tick cycles. No one "designed" the population cycles.

5. **Feedback loops:** Positive feedback (success breeds success) and negative feedback (scarcity drives conservation). The game is full of both, and kids learn to identify them.

6. **Scientific method:** Hypothesize → experiment → observe → update. The `ternary-science` journal system makes this explicit.

---

## 7. Multiplayer: Competitive Farming

### The Arena System

Multiplayer uses `ternary-arena` for competitive farming:

#### Head-to-Head Farming

Two players' farms compete on the same land with the same resources:

```
┌─────────────────────────────────────────┐
│         FARM ARENA: SHOWDOWN            │
│                                         │
│  Player A (left)  vs  Player B (right)  │
│  ┌──────┐              ┌──────┐        │
│  │ Farm │              │ Farm │        │
│  │  A   │              │  B   │        │
│  └──────┘              └──────┘        │
│                                         │
│  Shared Resources:                      │
│  💧 Water: 1000 units (shared)          │
│  ☀️ Sunlight: 2000 units (shared)       │
│  🌱 Soil: 500 nitrogen units (shared)   │
│                                         │
│  Scoring:                               │
│  📊 Yield + Diversity + Conservation    │
│  🏆 Best composite score wins           │
│                                         │
│  Conservation: ENFORCED                 │
│  (Shared resources must balance)        │
└─────────────────────────────────────────┘
```

Because resources are shared and conservation is enforced, one player's over-use directly harms the other. This creates genuine strategic tension: do you use water aggressively (maximizing your yield but depleting the shared supply) or conservatively (preserving resources but potentially falling behind)?

#### Tournament Mode

Multiple farms compete in elimination brackets:

1. **Qualifying:** All farms run independently. Top 50% by composite score advance.
2. **Bracket play:** Paired farms compete head-to-head on shared resources.
3. **Finals:** Top 2 farms compete for the championship.

Each round introduces new constraints:
- Round 1: Standard farming.
- Round 2: Drought conditions (reduced water).
- Round 3: Pest invasion (ecosystem challenge).
- Round 4: Market crash (economic challenge).
- Finals: All constraints simultaneously.

#### Cooperative Mode

Not all multiplayer is competitive. In cooperative mode:

- Players specialize: one grows crops, another raises animals, a third manages the ecosystem.
- Resources flow between players through trade.
- The conservation law applies to the entire cooperative — total resources must balance across all players' farms.
- Success requires coordination and communication.

### The Arena as GPU Workload

On the backend, arena matches run on CudaClaw's GPU:

- Each match = 1 warp (32 threads for 2 farms' state).
- A 64-player tournament = 32 first-round matches = 32 warps.
- Full tournament (6 rounds) runs in <1 second on GPU.
- Results are reported back to the living spreadsheet for visualization.

---

## 8. Implementation Roadmap

### Phase 1: Single-Player Prototype (Weeks 1-8)

- Core game loop: plant → grow → harvest → sell → repeat
- `ternary-cell` agents as crops and animals
- `ternary-ecosystem` food web
- `conservation-verify` resource tracking
- Basic spreadsheet dashboard with `=FIT()`, `=CONSERVATION()`, `=ROLL()`
- One NPC advisor (Old Farmer Jeb)

### Phase 2: Breeding and Evolution (Weeks 9-16)

- `ternary-evolution` breeding system
- `ternary-dice` weather events
- `ternary-fitness` health scoring
- `ternary-sensor` + `ternary-kalman` farm sensors
- `=EVOLVE()` formula
- Three more NPC advisors

### Phase 3: Rigging and Exploration (Weeks 17-24)

- `ternary-rigging` what-if machine
- `=SHAKE()` formula for parameter exploration
- `ternary-bayesian` forecasting
- `=FORECAST()` formula
- `ternary-explain` explanations
- All five NPC advisors

### Phase 4: Multiplayer Arena (Weeks 25-32)

- `ternary-arena` competitive farming
- `=ARENA()` formula
- `ternary-game-theory` market dynamics
- `ternary-market` trading
- Tournament mode
- Cooperative mode
- CudaClaw GPU backend for arena matches

### Phase 5: Polish and Education (Weeks 33-40)

- `ternary-curriculum` progressive difficulty
- Tutorial levels (Minecraft → real physics transition)
- `ternary-science` journal system
- `ternary-explain` enhanced explanations
- Accessibility features
- Teacher dashboard for classroom use

---

*— Synthesis Agent*
*June 2026*
