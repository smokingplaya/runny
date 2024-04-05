<p align="center">
  <img src="images/runny.png">
</p>

<h4 align="center">scripting tool, for simpler projects development</h4>

# 🤔 How it works

Runny is command-line tool, that run start/build/etc commands
for your projects in workspaces.

Runny executes pre-recorded commands (presets) in your console/terminal.
For example Runny can be used to build your project.

[Example of a preset for building a Rust project.](./.runny/settings.yml)

```bash
runny # starts runny with "default" preset
runny test # start runny with "test" preset
```

# 🛠️ Building

You can build Runny for yourself.
But, you should have [Rust language](https://rust-lang.org)
on your building machine.

```
cargo run
```

# 🛸 List of availiable shells

* PowerShell (powershell)
* Bash (bash)
* Windows CMD (cmd)
* ZSH (zsh)


# 🐼 How to configure

Well, for every your project, in root dir you need to create
folder with name **.runny**, then you need to create **settings.yml** file.

### 👩‍🦼 First steps

Specify [shell](#list-of-availiable-shells) in which your commands will be executed:

```yaml
runner-default: bash
```

### Configure your presets

Preset is a HUINYAAA thats executed, ebat'

By default, if you running **runny** command in your shell, without argumenst,
Runny starts default preset.

```yaml
presets:
	default:
		command: ls
```

# 🐣 Contributing

soon