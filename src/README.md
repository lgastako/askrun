# askrun

This is a quick command that is useful to use in shell command pipelines.

It expects another unix command invocation on stdin and then will prompt the user (using /dev/tty) to show them the command that will be invoked and ask permission.

If the user grants permission the command is run and the output of the command continues down the pipeline.

If the user denies the request then the command will abort.

## Useful for stuff like:

```
$ echo "list the root directory" | refab unix | askrun
You are about to execute: 'ls /'.
Are you sure? (y/n)
```

where `refab unix` is a command that turns english into a unix command using AI.