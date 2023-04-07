# prompt
A little program that allows to storage some prompts right into .json file and then use it whatever you want.

```bash
$ prompt -c -- <prompt name> - create new prompt

$ prompt -r -- <prompt name> - delete already exists prompt

$ prompt -- <prompt name> - prints prompt by name
```
Don't forget to add an env variable:
```bash
$ export PROMPT_DIR_DEFAULT=<path/to/dir>
```
If that variable is specified, program should create a json file right into that folder.
