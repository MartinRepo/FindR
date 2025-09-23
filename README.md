# Findme 🎯

[🇨🇳 简体中文](README_CN.md)

A fun terminal command-line tool that provides daily decompression fortune predictions for programmers and tech professionals.

## Features

- 🎯 **Programmer's Daily Decompression Oracle** - Tech dimension analysis with deterministic daily variations
- 📊 **Tech Dimensions Scoring** - Five-dimensional analysis: Focus, Creativity, Debugging Touch, Collaboration Index, Risk Tolerance
- 🏢 **Smart Scenario Detection** - Automatically adapts to workday (execution/delivery) vs weekend (learning/exploration) modes
- 🎲 **Deterministic Generation** - Same birthday + same day = consistent results, but varies daily automatically
- 🎨 **Colored Terminal Output** - Beautiful colored interface with dimension visualization bars
- 💡 **Personalized Tech Advice** - Tailored recommendations based on your tech dimensions and current scenario
- 🔬 **Developer Pressure Index** - Analyzes local git/test/build data for risk and patience thresholds
- 🌍 **Multi-language Support** - Support for Chinese and English
- 🔐 **Birthday-based Personalization** - Optional birthday input for personalized analysis
- 💾 **Local Preferences Storage** - Remembers your saved language and birthday for future sessions

## Tech Dimensions

Findme analyzes your daily tech performance across five key dimensions:

| Dimension | Description | Workday Focus | Weekend Focus |
|-----------|-------------|---------------|---------------|
| 🎯 **Focus** | Concentration and attention to detail | High (40-100) | Medium (20-70) |
| 💡 **Creativity** | Innovation and problem-solving ability | Medium (20-80) | High (60-100) |
| 🐛 **Debugging** | Troubleshooting and error-fixing skills | High (50-100) | Medium (30-80) |
| 🤝 **Collaboration** | Teamwork and communication effectiveness | High (60-100) | Low (10-50) |
| ⚡ **Risk Tolerance** | Willingness to try new approaches | Low (10-60) | High (50-100) |

### Scenario-Based Weighting

- **Workday Mode**: Focuses on execution and delivery (Focus 35%, Debugging 30%, Collaboration 25%, Creativity 7%, Risk 3%)
- **Weekend Mode**: Emphasizes learning and exploration (Creativity 40%, Risk 30%, Focus 20%, Debugging 7%, Collaboration 3%)

## Installation

### From Source

```bash
git clone <repository-url>
cd findme
cargo build --release
```

### Install to System

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Show today's tech fortune (auto-detects workday/weekend)
findme

# Show detailed information and today's fortune
findme --verbose

# Personalized analysis with birthday
findme --birthday "1990-05-15"
# The birthday will be cached locally for next time

# Show developer pressure index (analyzes local git/test/build data)
findme --pressure

# Combine features
findme --pressure --birthday "1990-05-15" --language zh

# Specify language
findme --language zh  # Chinese
findme --language en  # English

# Set default language
findme --set-language
```

### Language Setup

On first use, the tool will prompt you to select a language. You can also change the language setting anytime:

```bash
# Set language
findme --set-language

# Temporarily use different language
findme --language en
```

### Example Output

#### Workday Mode
```
============================================================
🎯 Developer's Daily Decompression Oracle
📅 Workday Mode - Execute/Deliver
============================================================

📊 Overall Score: 72

🎯 Tech Dimensions
  🎯 Focus: ██████████████░░░░░░  74
  💡 Creativity: ███████████████░░░░░  75
  🐛 Debugging: ████████████░░░░░░░░  63
  🤝 Collaboration: ████████████████░░░░  84
  ⚡ Risk Tolerance: ████░░░░░░░░░░░░░░░░  20

💬 Today's Status: 👍 Good state today
💡 Tech Advice: High programming efficiency, but pay attention to code review, avoid small errors.

🎨 Recommended Color: Blue
⏰ Best Time: 5-6 PM

============================================================
```

#### With Developer Pressure Index
```
============================================================
🎯 Developer's Daily Decompression Oracle
📅 Workday Mode - Execute/Deliver
============================================================

📊 Overall Score: 72

🎯 Tech Dimensions
  🎯 Focus: ██████████████░░░░░░  74
  💡 Creativity: ███████████████░░░░░  75
  🐛 Debugging: ████████████░░░░░░░░  63
  🤝 Collaboration: ████████████████░░░░  84
  ⚡ Risk Tolerance: ████░░░░░░░░░░░░░░░░  20

💬 Today's Status: 👍 Good state today
💡 Tech Advice: High programming efficiency, but pay attention to code review, avoid small errors.

🎨 Recommended Color: Blue
⏰ Best Time: 5-6 PM

============================================================

============================================================
🔬 Developer Pressure Index
============================================================
🟡 Pressure Level Medium Pressure - Normal

📊 Development Metrics
  📝 Git Diff Lines 255 lines
  🧪 Test Success Rate 100.0%
  ⚡ Build Time 0s

💭 Today's Advice
  ⚠️ Medium Risk Threshold - Proceed with caution
  💡 Suggest improving existing features first, then consider new ones
============================================================
```

## Technical Implementation

- **Rust** - High-performance systems programming language
- **clap** - Command line argument parsing
- **chrono** - Date and time handling with weekday detection
- **rand_chacha** - Deterministic random number generation using ChaCha20
- **sha2** - SHA256 hashing for seed generation
- **serde** - Serialization framework for future template system
- **tinytemplate** - Template engine for dynamic content generation
- **colored** - Terminal color output with dimension visualization
- **dirs** - Configuration file path management

## Fortune Algorithm

Findme uses a deterministic algorithm that ensures consistency while providing daily variation:

### Seed Generation
```
seed = SHA256(birthday_YYYYMMDD + today_YYYYMMDD + version + salt)
```

### Deterministic Properties
- **Consistency**: Same birthday + same day = identical results
- **Daily Variation**: Different results each day automatically
- **Personalization**: Birthday input creates unique patterns
- **Scenario Awareness**: Automatically detects workday vs weekend

### Tech Dimension Generation
- Uses `ChaCha20Rng::seed_from_u64()` for deterministic randomness
- Scenario-biased ranges ensure realistic work patterns
- Weighted scoring adapts to workday/weekend contexts

## Configuration

Preferences are saved in `~/.findme/config.txt` as simple key-value pairs:

```
language=en
birthday=1990-05-15
```

- `language` is stored when you run `findme --set-language`
- `birthday` is automatically saved the first time you pass `--birthday`

To customize the location (for example in scripts or automated tests), set the
`FINDME_CONFIG_DIR` environment variable to the directory where the config file
should live.

## License

MIT OR Apache-2.0

## Contributing

Welcome to submit Issues and Pull Requests!

---

**Note**: This is an entertainment tool, fortune predictions are for entertainment purposes only, please make work decisions rationally.