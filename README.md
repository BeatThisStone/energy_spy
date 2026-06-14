# energy_spy

energy_spy is a hachimi plugin to dump data at runtime.

## Installation

1. Install hachimi

Download and install Hachimi Edge from:

`https://github.com/kairusds/Hachimi-Edge/releases/latest`

2. Download `energy_spy.dll`

Grab the latest `energy_spy.dll` from:

`https://github.com/BeatThisStone/energy_spy/releases/latest`

Place it in your game folder root.

3. Enable the plugin in hachimi

Open `hachimi/config.json` in your game folder and add `energy_spy.dll` to `load_libraries`:

```json
{
  "load_libraries": [
    "energy_spy.dll"
  ]
}
```

## Known Issues

- Game crashes if the header containing the energy bar was loaded at least once and is not loaded when the user clicks the "Show Energy" button