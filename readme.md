```{
// See https://go.microsoft.com/fwlink/?LinkId=733558
// for the documentation about the tasks.json format
"version": "2.0.0",
"tasks": [
{
"label": "cargo run",
"type": "shell",
"command": "cargo run",
"group": {
"kind": "build",
"isDefault": true
},
"problemMatcher": [
"$rustc"
]
},
{
"type": "cargo",
"subcommand": "build",
"problemMatcher": [
"$rustc"
],
"group": {
"kind": "build",
"isDefault": true
}
},
{
"type": "cargo",
"subcommand": "check",
"problemMatcher": [
"$rustc"
]
},
{
"type": "cargo",
"subcommand": "clean",
"problemMatcher": [
"$rustc"
]
},
{
"type": "cargo",
"subcommand": "test",
"problemMatcher": [
"$rustc"
]
}
]
}
```
