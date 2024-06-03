# TouchDesigner plugins

This repository is a fork of the awesome [td-rs](https://github.com/tychedelia/td-rs) with more plugins added.

## Installation
You can download pre-built versions of the plugins from the [releases](https://github.com/thatmarcel/touchdesigner-plugins/releases).

Then, move or extract the downloaded file to the TouchDesigner plugins directory.
The default paths for macOS and Windows are currently the following (you may need to create the `Plugins` folder yourself):
- **Windows**: `C:\Users\<your username>\Documents\Derivative\Plugins\`
- **macOS**: `/Users/<your username>/Library/Application Support/Derivative/TouchDesigner099/Plugins`

Alternatively, you can build and install the plugin you want yourself by following the [instructions on the original repo](https://github.com/tychedelia/td-rs?tab=readme-ov-file#build).

You can find the new operators under the `Custom` tab in the **OP Create Dialog** in TouchDesigner.

## Plugins
### Lasercube
A plugin that allows you to control a Lasercube via Wifi.
It takes the output of a Laser CHOP as the input.

Make sure the sample rate of the Laser CHOP is equal to that of the Lasercube CHOP and that it doesn't exceed the device's max sample rate.

This plugin is still in development. You may experience visual glitches or disconnects.

### Unreal Engine Connection
A plugin that allows you to send values from Touch Designer to Unreal Engine and vice-versa in realtime.

#### Setup
There is a more detailed and complete guide with videos available in [English](https://guides.thatmarcel.com/guides/sending-values-between-touch-designer-and-unreal-engine) and [German](https://guides.thatmarcel.com/guides/in-echtzeit-daten-zwischen-touch-designer-und-unreal-engine-schicken).

1. Install the plugin as described above
2. Open or create a TouchDesigner project and place an `Unreal Engine Connection` CHOP.
3. In the parameters for the CHOP, click `Add` next to `Callback DAT` (you don't need to do anything with that DAT but it's needed by the plugin as a workaround for reading the input channel names).
4. Open or create a project in Unreal Engine (tested with UE 5.4.1 but other versions may work as well)
5. Create a new C++ class via the `Tools` menu to create a C++ project if you don't have one already
6. Download the contents of [the repository with the files for the Unreal Engine side](https://github.com/thatmarcel/unreal-engine-touch-designer-connection)
7. Move the `TouchDesignerConnection` folder from the downloaded repository into the `<UE project location>/Source/<UE project name>` folder (there should also be the files `<UE project name>.cpp` and `<UE project name>.Build.cs` in there).
8. Add `"Networking"` and `"Sockets"` to the list of `PublicDependencyModuleNames` in the `<UE project name>.Build.cs` file
9. Open `<UE project name>/Source/<UE project name>/Public/<the class you created earlier>.h`. Copy `<whatever>_API` after the `class` keyword (probably in line 10).
10. Open `<UE project name>/Source/<UE project name>/TouchDesignerConnection/TouchDesignerConnectionActor.h` and replace `BLANKTEST1_API` with what you copied.
11. Compile your Unreal Engine C++ project
12. Find the `TouchDesignerConnectionActor` in the `C++ Classes/<UE project name>/TouchDesigner` folder inside the Unreal Engine **Content Browser** and drag it into the level view to place it in the world.
13. You're now ready to send values between TouchDesigner and Unreal Engine (you need to run the Unreal Engine project to establish a connection)
    - To send values from TD to UE, attach a CHOP with one or more channels to the input of the `Unreal Engine Connection` CHOP. The values will now appear under `Received Values` on the `TouchDesignerConnectionActor` instance. You can e.g. access these values via blueprints.
    - To send values from UE to TD, add entries to `Sent Values` on the `TouchDesignerConnectionActor` manually or e.g. via a blueprint. The output of the `Unreal Engine Connection` CHOP will then contain the values.
    - (You can also have TD and UE running on different computers in the same network as long as you adjust the IP addresses accordingly)
    - (If you want to run multiple instances of the plugin, you can change the ports. Make sure they are the same in each TD-UE pair but no two pairs use the same ports)