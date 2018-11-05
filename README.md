# set_eq: A command-line tool to manipulate PulseAudio's equalizers

[![dependency status](https://deps.rs/repo/github/minijackson/set_eq/status.svg)](https://deps.rs/repo/github/minijackson/set_eq)

This tool will allow you to use a configuration file to send to your favorite
PulseAudio's equalizer.

It was originally made to work with
[AutoEq](https://github.com/jaakkopasanen/AutoEq)'s
[database](https://github.com/jaakkopasanen/AutoEq/tree/master/results), but if
you have ideas on how to extend `set_eq`'s domain, please start a discussion in
a new issue!

Supported configuration format:

- EqualizerAPO (very partial)

Supported equalizer:

- PulseAudio's built-in equalizer (deprecated and buggy, should not use)
- PulseEffects

## Quick start

- Install [PulseEffects](https://github.com/wwmm/pulseeffects)
- Find or create an EqualizerAPO configuration file
	- If you have a some headphones, try to find it
	  [here](https://github.com/jaakkopasanen/AutoEq/tree/master/results)
	  and either download the `<headphone> GraphicEQ.txt` file or save the
	  EqualizerAPO snippet from the README
- Run the following command:

```
set_eq pa-effects export-preset <EqualizerAPO file> > MyPreset.json
```

- Load the new preset in PulseEffects' interface
- Profit!
