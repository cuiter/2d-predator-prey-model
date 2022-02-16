# uu-onderzoeksmethoden

Source code and appendices for the course Onderzoeksmethoden voor de Informatica (B3OMI) at the UU

# Problem definition

![](design/problem-definition.png)

# Requirements

The goal of this program is to do the following:

- Provide a platform for building and simulating a grid-based model of cells.
- Collect statistics from the simulated model in order to discover useful results.

By splitting these goals up into requirements, the following list can be formed:

Must:

- Provide a platform for building and simulating a grid-based model
- Allow the user to specify model parameters at the beginning of the simulation
- Show the current state of the model in a graphical user interface
- Allow the user to control the time aspect of the model at runtime (play/pause/speed)
- Collect and write model statistics to a file (including #cells per cell type)

# Design

![](design/component-diagram.png)

The goal of the course is to find a problem that can be expressed in relatively
simple terms, use a model to simulate the problem, and be able to say something
interesting about its results.

Because the exact problem definition is not fixed yet, the main goal of the
design is to provide a platform with which to experiment and compare different
solutions.
