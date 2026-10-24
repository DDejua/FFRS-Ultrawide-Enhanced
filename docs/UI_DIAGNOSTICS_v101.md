# v1.0.1 UI state investigation (experimental)

This is **not a release**. It traces selected Unreal UMG widget visibility
events and rebuilds to investigate the independently repositioned minimap
appearing beside 16:9 menus in HYBRID mode.

## Reproduction (unpatched 1.0.0)

* HYBRID 3440x1440: Y / Esc main menu — minimap visibly leaks at right.
* HYBRID 3440x1440: J / RB Missions screen — same leak.
* World Map (M) and battle — observed normal.
* CLASSIC/ULTRAWIDE — separate layout behavior.

## Test source

Only \`overrides/src/fixes/mod.rs\` has tracing additions.
The existing \`minimap.rs\` reposition logic is unchanged.
The tracing runs only with \`[minimap] probe = true\`.
This candidate does **not** fix the leak.

The \`SetVisibility\` hook logs \`UI_DIAG_VIS\` for matching layout/menu/map
classes and stack names, only when the requested visibility differs from the
widget's current value. The widget rebuild hook logs \`UI_DIAG_REBUILD\`
once per matching instance. Diagnostic keys are capped to limit growth.

An absent trace is **not** proof that no menu was opened: CommonUI can change
active layers without calling the hooked \`SetVisibility\` path.

## Building

GitHub Actions: Build Windows candidate (not a release). Choose branch
\`test/menu-diagnostics-v101\` if running manually. This branch also has a
temporary push trigger for its own diagnostic commits; **main is unchanged**.
The build pins PolarWizard version 0.3.0 and Rust 1.99.1.

The resulting workflow artifact contains
\`upstream/target/release/final_fantasy_resonance_fix.dll\`, not a packaged
end-user installer.

## Testing locally on Windows

1. Backup saves, current \`scripts/final_fantasy_resonance_fix.asi\` and TOML.
2. Exit the game. Keep the already-installed \`winmm.dll\`.
3. Copy the **compiled experimental DLL** into \`FFRS/Binaries/Win64/scripts/\`
   as \`final_fantasy_resonance_fix.asi\`.
4. Set the **installed** TOML's \`[minimap]\` section to:
   \`\`\`toml
   [minimap]
   probe = true
   enable = true
   name_hint = "WBP_MiniMapLayout_C"
   \`\`\`
   Restart the game after editing.
5. Reproduce exploration > main menu > close > Missions > close > world map >
   close > battle > return to exploration.
6. Exit game. Check \`FFRS/Binaries/Win64/scripts/final_fantasy_resonance_fix.log\`.
   Search for \`UI_DIAG_\` and \`MINIMAP_\` lines. Correlate their timestamps.
7. **Redact file paths, account names and machine-specific data** before
   sharing logs. The base logger writes full paths.
8. Restore the known-good \`.asi\` and TOML.

Do not publish or redistribute the diagnostic build as v1.0.1.
