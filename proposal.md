# Aquamarine Toolkit

## Overview

Aquamarine Toolkit is a set of utilities that will
enhance the user experience of people using multiple
computers. While our foremost intention is to
provide utilities for host / virtual machine
environments, the structure of the application
should not prohibit several physical computers as
well.

Several basic features that we'd like to include in
our minimum viable product are:

- Open links on other machine
- Synchronize clipboard
- File transfer (following KISS Principles)
- Cross Platform compatibility

Additional features that have a lower priority might
include:

- FUSE Mounts of cross-system files
- Execution of commands / macros on other machines

## Semester Plan

At first, the plan will be to setup repositories
with the necessary open source documentation, such
as a code of conduct, and create project boards to
better visualize our intended features. Then, we
will move into implementation by creating an
application following client/server model that can
connect via TCP to the reciprocal application.

At this point, features should be implemented.

## Technology

Being system-level daemon applications, we will be
using the systems programming language Rust, along
with relevant open source libraries ("crates") to
develop the toolkit.

## Team

| **Name** | **GitHub Handle** | **Email** |
|:------:|:-------:|:------:|
| Kate Vandermolen | [@CerulanLumina](https://github.com/CerulanLumina) | kate@katevandermolen.dev | 

## Milestones

- End of week 1: repository, code of conduct, project boards created
- Week 3: Rust project boilerplate / TCP model complete
- Week 5: Several Features implemented, ideally MVP (see above)
- Week 8: Additional features with lower priority (see above)

*End of Semester is at +8 weeks I believe? Trying to encapsulate the whole project in a single semester.* 
