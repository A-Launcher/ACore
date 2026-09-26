# ACore architecture

ACore is the lowest-level shared contract crate in the A Launcher ecosystem.

## Responsibilities

ACore owns small, stable definitions that multiple components need to agree on:

- public API/version contracts
- platform and CPU architecture identifiers
- launch request/result data structures
- shared core error types

## Non-responsibilities

ACore does **not**:

- download Minecraft files
- resolve Minecraft versions or libraries
- install or build a JVM
- implement JNI
- implement LWJGL, GLFW, OpenGL, Vulkan, or OpenAL
- contain Android UI
- start Minecraft itself

Those responsibilities belong to dedicated repositories.

## Dependency direction

The intended dependency direction is:

~~~text
Android UI / Launcher -> ACore <- Runtime / Native components
~~~

ACore should remain dependency-light so that native and higher-level components can use it without pulling the entire launcher stack into their builds.

## Launch contract

A launcher eventually resolves a Minecraft installation into a LaunchRequest.

The request contains:

1. game directory
2. Java executable
3. Minecraft main class
4. JVM arguments
5. classpath entries
6. program arguments

The component responsible for execution consumes that request and returns a LaunchResult.

## Compatibility rule

When a higher-level repository needs a new cross-component concept, prefer adding a small explicit contract to ACore rather than exposing implementation details from another repository.

Breaking a public contract requires a major ACore API version change once the API reaches 1.0.
