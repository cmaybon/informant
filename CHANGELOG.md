# 0.1.4
## Fixes
- Fixed issue where dates where 1 month behind (fixing issue ["X Axis label displays the incorrect month"](https://github.com/cmaybon/informant/issues/4))

## Changes
- Added some unit tests to `stats.rs`

---

# 0.1.3
## Features
- Added loading of `todaystats` to show today's stats
    - Tries to autoload path from AppData
    - Added setting in `SettingsTab`

## Changes
- Updated `SettingsTab` ui layout
- `icon.png` is now packaged into the binary

---

# 0.1.2
## Changes
- Added an `exe` icon on Windows
- Added application icon

---

# 0.1.1
## Features
- When no `historystats` path is set attempts to use `C:\Users\<username>\AppData\Roaming\Workrave`
- `historystats` is now attempted to be loaded on launch
- `historystats` is now cached once loaded
- Added button to `StatsTab` to reload `historystats` when loads fail 

## Changes
- Removed current day stats placeholder UI
- Plots now scale with screen size properly
- Added headings above all plots
- Removed `Load Stats` button from top panel
