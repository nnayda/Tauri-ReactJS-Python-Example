# Tauri + React + Typescript + Python

Example app showing how to connect a backend python service to a Tauri app.

## Installation

 - `yarn install`
 - Development:
    - `yarn build_py` - build changes to python files
    - `yarn tauri dev` - build/launch tauri app
 - Production:
    - `yarn tauri build`

Note you will need to have Python installed with the following dependencies:
 - `pyinstaller`
 - `uvicorn`
 - `fastapi`
