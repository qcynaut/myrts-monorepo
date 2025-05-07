# MYRTS (Legacy)

This repository contains all the code used in developing the MYRTS project.
Before this, the repository was separated into multiple repositories.
But, for the sake of simplicity, this repository is now a single repository (monorepo).

## Reason for the monorepo

After further investigation and considering the benefits of the monorepo, I (Ade M Ramdani) decided to use the monorepo.
Disadvantages of the last structure are:

- It requires a lot of time to develop the code.
- Hard to share the code and force the developer to re-create the code that is already there.
- The structure of the project is not very clear.
- Hard for the new developer or team to understand the code or the workflow.

## Conclusion

From now on, this repository will be the main repository for the MYRTS project. And give a better understanding of the structure of the project.

## Structure of the project

- [README.md](README.md) - This is the README of the project.
- [.circleci](.circleci) - This is the CircleCI configuration file.
- [rust](rust) - All the Rust-related code lives in this folder as rust workspace.
- [web](web) - All the web-related code lives in this folder i.e: `web-frontend`, `typescript-library`, etc.
- [mobile](mobile) - All the mobile-related code lives in this folder i.e: `mobile-frontend`, `dart`, `flutter`, `swift`, etc.
