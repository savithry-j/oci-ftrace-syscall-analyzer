# oci-ftrace-syscall-analyzer

## Introduction

`oci-ftrace-syscall-analyzer` is an experimental tool to trace syscalls inside containers without any debug tools in Pod. This tool uses ftrace to debug containers transparently.

You can launch the rootless container using this tool when you use SUID or modify permission to the directories of tracefs.

## Usage

### From low level runtime

Yo need to add the prestart and poststart hook in config.json.
```json
  "hooks": {
    "prestart": [
      {
        "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
        "args": [
          "oci-ftrace-syscall-analyzer",
          "record"
        ]
      }
    ],
    "poststop": [
      {
        "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
        "args": [
          "oci-ftrace-syscall-analyzer",
          "report"
        ]
      }
    ]
  },
```

### From high level runtime

(WIP) You need to add OCI hook settings.

#### CRI-O integration sample

(WIP) prestart setting exmaple for oci-hooks
```json
$ cat /etc/containers/oci/hooks.d/ftrace-syscall-analyzer-hook.json
{
  "version": "1.0.0",
  "hook": {
    "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
    "args": [
      "oci-ftrcae-syscall-analyzer",
      "record",
      "--use-annots"
    ]
  },
  "when": {
    "annotations": {
      "oci-ftrace-syscall-analyzer/trace": "true"
    }
  },
  "stages": [
    "prestart"
  ]
}
```

### From Kubernetes

(WIP) You can pass options to high level runtimes using annotations.
