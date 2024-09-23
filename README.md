
# Codify

An app that simplifies task automation and presents it super clearly.

It automatically converts automations to buttons with forms for input data etc.


## Features

- Light/dark mode toggle
- Cross platform
- Automation tasks turn into buttons
- File preview
- All automations in one place


## Demo

All Automations are written in a YAML file

```yaml
# All subfolders will be made in this directory
baseLocation: C:/Users/dell/Documents/projects/dev
# These are the subfolders/ subcategories in which projects are made
# The 'name' will be used in a templates' 'type' argument
folders:
  - name: node.js
    folder: 'nodejs'
# Just sequences of actions to do
# Eg. Setup workspace by opening Chrome, VS Code, Terminal etc.
actions:
  - name: demo action
    arguments: # arguments are fields which require manualinput in order to run the action
      - name
    commands:
      - echo this is a sample action
      - echo hello $name #argument is specified like this
  - name: demo2
    arguments:
      - age
    commands:
      - echo Your age is $age
# Here, templates for projects are made
# They will be automatically be setup since u specify what the category of the project is and the steps
# A new folder with the name specified will automatically be made and all commands are run in it
projectTemplates:
  - name: Basic node.js
    arguments:
      - name
      - type
    commands:
      - npm init -y
```

On running the app, this gets read and rendered into the UI.

...TODO
## Documentation

TODO

## Authors

- [@Akshay-VK](https://www.github.com/Akshay-VK)


## Acknowledgements

 - [Tauri](https://tauri.dev)
 - [SvelteKit](https://kit.svelte.dev)
 - [Material UI for Svelte](https://ktibow.github.io/m3-svelte/)


## License

[GNU General Public License v3.0](https://github.com/Akshay-VK/Codify/blob/master/LICENSE)

