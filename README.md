# oci-ftrace-syscall-analyzer

## Introduction

- `oci-ftrace-syscall-analyzer` is an experimental tool to trace syscalls inside containers without any debug tools in Pod. This tool uses ftrace to debug containers transparently.
- Overview of `oci-ftrace-syscall-analyzer` is explained on https://speakerdeck.com/kentatada/debug-application-inside-kubernetes-using-linux-kernel-tools

## Requirements

- You need to mount tracefs to use this tool.
- You can launch the rootless container using this tool when you set the Linux capability of `CAP_DAC_OVERRIDE` to the binary of oci-ftrace-syscall-analyzer or modify permission to the directories of tracefs

## Install from Relese page

- Please download the binary from the release page for amd64 like below.
```
$ wget https://github.com/KentaTada/oci-ftrace-syscall-analyzer/releases/download/v0.1.1/oci-ftrace-syscall-analyzer.amd64
$ sudo cp oci-ftrace-syscall-analyzer.amd64 /usr/local/bin/oci-ftrace-syscall-analyzer
$ sudo chmod a+x /usr/local/bin/oci-ftrace-syscall-analyzer
$ sudo setcap CAP_DAC_OVERRIDE+ep /usr/local/bin/oci-ftrace-syscall-analyzer
```
- You can build oci-ftrace-syscall-analyzer from the source code for architecture other than amd64.

## Install from Source

### On Linux

#### Install and setup rust with rustup

- Building oci-ftrace-syscall-analyzer requires rustc version 1.32.0 or later.
- If you don't have rust yet, run:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- If you have an old version, run:
```
$ rustup update
```

#### Getting and Building the oci-ftrace-syscall-analyzer Sources

- Clone sources from git repo
```
$ git clone https://github.com/KentaTada/oci-ftrace-syscall-analyzer.git
```
- Build sources
```
$ cd oci-ftrace-syscall-analyzer
$ cargo build --release
```
- Install binary
```
$ sudo cp target/release/oci-ftrace-syscall-analyzer /usr/local/bin/
$ sudo chmod a+x /usr/local/bin/oci-ftrace-syscall-analyzer
$ sudo setcap CAP_DAC_OVERRIDE+ep /usr/local/bin/oci-ftrace-syscall-analyzer
```

### (Advanced) Building as a Static Binary

- The binary built by the above method is dynamically linked binary. But the released binary is a static binary.
- If you want to build static binary, you need to get MUSL support.
 - For example, on x86_64 architecture, run:
```
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl
```

## Usage

- This tool is only executed inside container hooks without report --livedump option.
- The annotation is used to control trace settings from Kubernetes. When you set the key of "oci-ftrace-syscall-analyzer/trace" to "true" with --use-annots option, oci-ftrace-syscall-analyzer is executed.

### From low level runtime

You need to add the prestart and poststop hook in config.json.
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

#### e.g. How to integrate oci-ftrace-syscall-analyzer with rootless runc

##### 1. Set the Linux capability of `CAP_DAC_OVERRIDE` to oci-ftrace-syscall-analyzer. Or change permission to the directories of tracefs.

```
# setcap CAP_DAC_OVERRIDE+ep /usr/local/bin/oci-ftrace-syscall-analyzer
```

##### 2. Prepare for the sample container

```
$ mkdir rootless_trace
$ cd rootless_trace
$ mkdir rootfs
$ docker export $(docker create busybox) | tar -C rootfs -xvf -
$ runc spec --rootless
```

##### 3. Modify config.json to use hook

Add below to config.json.

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

##### 4. Launch rootless container

```
$ runc run 1
/ # ls
bin   dev   etc   home  proc  root  sys   tmp   usr   var
/ # exit
```

##### 5. Confirm syscall logs

```
$ tail -n 10 ftrace_syscalls_dump.log
```

### From high level runtime

(WIP) You need to add OCI hook settings.

#### CRI-O integration sample

(WIP) You need to prepare for the prestart and poststop settings in the CRI-O's oci-hooks.
```
$ cat /etc/containers/oci/hooks.d/ftrace-syscall-analyzer-prehook.json
{
  "version": "1.0.0",
  "hook": {
    "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
    "args": [
      "oci-ftrcae-syscall-analyzer",
      "record"
    ]
  },
  "when": {
    "always": true
  },
  "stages": [
    "prestart"
  ]
}
$ cat /etc/containers/oci/hooks.d/ftrace-syscall-analyzer-posthook.json
{
  "version": "1.0.0",
  "hook": {
    "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
    "args": ["oci-ftrcae-syscall-analyzer", "report", "--output", "/tmp/syscalllogs/ftrace_syscalls_dump.log"]
  },
  "when": {
    "always": true
  },
  "stages": ["poststop"]
}
```

#### e.g. How to integrate oci-ftrace-syscall-analyzer with Podman using annotations

##### 1. Set the Linux capability of `CAP_DAC_OVERRIDE` to oci-ftrace-syscall-analyzer. Or change permission to the directories of tracefs.

```
# setcap CAP_DAC_OVERRIDE+ep /usr/local/bin/oci-ftrace-syscall-analyzer
```

##### 2. Modify Podman OCI hook settings

- Modify "hooks_dir" of libpod.conf to specify the directory includes OCI hook configs
```
hooks_dir = ["/etc/containers/oci/hooks.d"]
```
- Create OCI hook configs for oci-ftrace-syscall-analyzer
```
$ cat /etc/containers/oci/hooks.d/ftrace-syscall-analyzer-prehook.json
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
$ cat /etc/containers/oci/hooks.d/ftrace-syscall-analyzer-posthook.json
{
  "version": "1.0.0",
  "hook": {
    "path": "/usr/local/bin/oci-ftrace-syscall-analyzer",
    "args": ["oci-ftrcae-syscall-analyzer", "report", "--output", "/tmp/syscall_logs/ftrace_syscalls_dump.log"]
  },
  "when": {
    "annotations": {
        "oci-ftrace-syscall-analyzer/trace": "true"
    }
  },
  "stages": ["poststop"]
}
```

##### 3. Launch the sample container using Podman

```
# podman run --annotation oci-ftrace-syscall-analyzer/trace="true" docker.io/alpine:latest /bin/ls
```

##### 4. Confirm syscall logs

```
# tail -n 10 /tmp/syscalllogs/ftrace_syscalls_dump.log
```

### From Kubernetes

(WIP) You can pass options to high level runtimes using annotations.

### Live dump option

You can also get syscall logs while container is running. You can get the running container ID from some commands like kubectl describe pods and runc list and so on.
```
$ oci-ftrace-syscall-analyzer report --livedump [container-id]
```

### (WIP) seccomp profile generator

--seccomp-profile option generates seccomp profiles by the traced syscalls.
**Notice: In crrent, generated profile is not accurate. The profile does not include syscalls that invoked from start of trace to start of container app.**

#### e.g. How to generate profile with runC

##### 1. Modify config.json to use hookdump seccomp-profile

Add below to config.json.

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
	  "--seccomp-profile"
        ]
      }
    ]
  },
```

##### 2. Launch container

e.g. Execute ls command in container.
```
$ runc run 1
bin   dev   etc   home  proc  root  sys   tmp   usr   var
```

##### 3. Confirm syscall logsseccomp profile

```
$ cat seccomp.json
{
  "defaultAction": "SCMP_ACT_ERRNO",
  "syscalls": [
    {
      "names": [
        "arch_prctl",
        "brk",
        "close",
        "execve",
        "exit_group",
        "fstat",
        "futex",
        "getdents64",
        "getuid",
        "ioctl",
        "newfstatat",
        "open",
        "stat",
        "time",
        "write"
      ],
      "action": "SCMP_ACT_ALLOW"
    }
  ]
}
```

##### 4. Add logsseccomp profile to config.json

```
$ cat config.json

~~~~~

"linux": {
  "seccomp": {
    "defaultAction": "SCMP_ACT_ERRNO",
    "syscalls": [
      {
        "names": [
          "arch_prctl",
          "brk",
          "close",
          "execve",
          "exit_group",
          "fstat",
          "futex",
          "getdents64",
          "getuid",
          "ioctl",
          "newfstatat",
          "open",
          "stat",
          "time",
          "write"
        ],
        "action": "SCMP_ACT_ALLOW"
      }
    ]
  },

~~~~~

$ runc run 1
bin   dev   etc   home  proc  root  sys   tmp   usr   var
```