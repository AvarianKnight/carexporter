# carexporter

This is a simple resource for exporting vehicle/tires names so they're not NULL when in game.

All you have to do is go to exporter -> and double click 'exporter.exe' click 'Open File' and choose the directory you want to run it in, you can also just run it in the resources folder, though it will take longer as it has to go through every directory/file.

This will go through the directories you selected to generate the game data from the meta files, this can be instant, or take multiple minutes depending on how many cars you have.

You can also run this resource in 'headless' mode, just change the `headless.bat` path to the one you want it to run in.

This resource doesn't have any support.


This is just some stuff I want to implement in the future, as this was the primary thing that I've learnt rust from
## TODO's
- Maybe switch to quick-xml, might not make much of a difference as most of the performance issues currently are from seeking files.
- Implement tokio for async file system operations, will require signaling between threads.
- Cleanup code as I learn more
