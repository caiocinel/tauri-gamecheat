## This was just an experiment, it works to some extent, but it is unfeasible for several reasons:

- Canvas high memory usage
- High CPU usage on redraw
- Very weak documentation, community and tools related to Game Hacking in the Rust environment

In this basic example, it was possible to reach 60fps while drawing an ESP (which is not 100%, but it served the purpose of the test), but as it becomes necessary to increase the number of information that must be sent to the frontend, this gets exponentially worse (React gets even worse).

When it comes to overlay, it is still much more viable to use imgui due to the amount of iops, but if no overlay is necessary, it is currently completely possible to use only Tauri.

There is no comparison with Electron, which was already discarded just because of the package size.
