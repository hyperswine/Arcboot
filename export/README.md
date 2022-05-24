# Arcboot API

Separate API that can be linked to for arcboot kernels.

How to use: basically, specify arcboot to link too. And add that to workspace. Specify no default features and features = ["api"]. Now you can use `arcboot_api::*` in main.rs.
