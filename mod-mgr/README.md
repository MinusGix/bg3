# BG3 Rust Mod Manager
This is a reimplementation of LaughingLeader's [BG3ModManager](https://github.com/LaughingLeader/BG3ModManager) in RUst.  
The primary goal for this is to be able to run it on Linux (and MacOS?) devices, since BG3ModManager uses WPF and I've struggled to get it running under wine.

## Implemented features
- Settings UI
- Basic table ui

## TODO
- SteamDeck
  - I don't actually own one, so I'll have to rely on others to test/report problems.
  - Ensure that scrolling works right, I'm not sure the lib I'm using is listening for touchdrag events..
    - At worst, we can implement some weird pagination.
  - Are the various sizes good or at least decent?

- Scrolling in the settings for small height lets bottom options get cut off.                                                                                                                                                                                                                                                                                                                                                                                                                        

## Notable Missing Features
- Basically everything
- Screen reader support
- Saving settings
- Profile
- Actually loading load orders and exporting them
- Launching the game

## Credits
- LaughingLeader's [BG3ModManager](https://github.com/LaughingLeader/BG3ModManager) in RUst.  
- Jakub Jankiewicz's Clarity Icons (CC-3.0)
  - Manually modified some of them to be light for the dark background. UI lib doesn't provide a trivial way to invert it.