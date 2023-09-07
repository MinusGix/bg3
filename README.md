# BG3 Utilities
This is a collection of utilities I've written for Baldur's Gate 3. Many of them are based on (or outright rewritten from) existing libraries/tools, which is noted in their respective readmes.  


- `dialoguev`: A basic dialogue viewer for the game. Currently limited.
- `loca`: Parses `loca`lization files. Based on Norbyte's parsing.
- `lsf`: Parses `lsf` files. Based on Norbyte's parsing.
- `lsj`: Parses `lsj` files. Based on Norbyte's parsing.
- `ls`: Parses `pak` files. Based around LSLib, though I've split the subparts into crates.
- `mod-mgr-lib`: Library functions for the mod manager.   
- `mod-mgr`: Based on LaughingLeaders BG3 Mod Manager. Intended to work on Linux, like a Steamdeck, and other operating systems. Currently it simply tracks changes made upstream. NOTE: DOES NOT WORK CURRENTLY. I started on this project but I eventually managed to get the C# mod manager to work on Linux. I might still continue this eventually, but it takes backseat compared to other stuff.  