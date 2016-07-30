VERY Rough draft

qa-autopilot
===

Helps with testing vid games. Record and plays back sessions of input then asks
the game to check if the end result is correct. Results are logged. It requies the
game developer to 'write tests' (setup, record inputs, result checking), but once
this work is done you can have the tests run over night and basically get some 
affordable and consistent testing done.

Overview
---

Crafting high-quality robust games is a tricky and long endeavour,
not just because programming it all is hard, but because consistent and good
feedback from QA isn't as simple to get as it may seem. Since many developers utilize
many different engines and platforms, it's important to keep the toolset as portable as
possible to help testing on multiple platforms. The aim to do this is using simple local
network connections and protobuf to serialize. (Protobuf has the added benefit of not 
breaking a developers test suite every time we release a patch).

Inspired by the tools that speedrunners, this project aims are creating an automated 
testing system that you can run overnight and come back the next morning to see which 
tests failed, and recorded footage on how the test failed (ideally the footage would 
include the inputs overlayed).

Similar to unit testing, this system requires an internal component that sets up a scene
(like loading a save), and signals an external program to start feeding it input, when 
the external program finishes it signals the internal component to check if the test 
passed or failed. The internal component then tells the external app so that the app can 
log and display the results. We want to keep as much functionality as possible outside of
the game itself so as to prevent interfering with tests.

1. Prepare test (this can be done however the game dev wants)
2. Playback input
3. Check game's state to determine pass or fail 

There are two key components:

1. The external app that feeds input into the game, and displays test status
    (current, failed, passed)
2. The internal component that signals the external app when to start and stop input, 
    then checks the results.

Responsibilities of external tool
---

- Record input
- Playback input
- Log and display test results
- Send test results via webhooks
- Record video of test
- Overlay inputs over video
- Send/recieve signals to/from internal component
- Some kind of UI frontend, probably a web interface since that's portable
- Can recive logging events (must be easy to integrate with unity's logger)

Responsibilites of internal component
---

- Framework for setting up test
- Send/recieve signals to/from external tool
- Simple enough that developers can implement it in their game 
    without us providing libraries

Platform priorities
---

1) Windows
2) OSX
3) SteamOS / Ubuntu

Cross-platform is important, but to start we'll target windows and branch out
once key features are working and have unit tests.

Roadmap
---

- Console app that acts as a simple server to connect to
- A game implementation that can connect to the console app
- Web interface to send signals to game, and see signals recieved
- Can record mouse and keyboard inputs
- Can playback m&k inputs
- Can record controller inputs
- Can play back controller inputs
- Example code for tests and implementation
- Example unity project, throughly annotated