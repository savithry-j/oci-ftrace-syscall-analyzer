pub struct Error {
    pub name: String,
    pub desc: String,
}

#[macro_export]
macro_rules! error_info {
    () => {{
        {
            let mut tmp = HashMap::new();
            tmp.insert(
                "0xffffffffffffffff",
                Error {
                    name: "EPERM".to_string(),
                    desc: "operation not permitted".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffffe",
                Error {
                    name: "ENOENT".to_string(),
                    desc: "no such file or directory".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffffd",
                Error {
                    name: "ESRCH".to_string(),
                    desc: "no such process".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffffc",
                Error {
                    name: "EINTR".to_string(),
                    desc: "interrupted system call".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffffb",
                Error {
                    name: "EIO".to_string(),
                    desc: "input/output error".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffffa",
                Error {
                    name: "ENXIO".to_string(),
                    desc: "no such device or address".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff9",
                Error {
                    name: "E2BIG".to_string(),
                    desc: "argument list too long".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff8",
                Error {
                    name: "ENOEXEC".to_string(),
                    desc: "exec format error".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff7",
                Error {
                    name: "EBADF".to_string(),
                    desc: "bad file descriptor".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff6",
                Error {
                    name: "ECHILD".to_string(),
                    desc: "no child processes".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff5",
                Error {
                    name: "EAGAIN".to_string(),
                    desc: "resource temporarily unavailable".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff4",
                Error {
                    name: "ENOMEM".to_string(),
                    desc: "cannot allocate memory".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff3",
                Error {
                    name: "EACCES".to_string(),
                    desc: "permission denied".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff2",
                Error {
                    name: "EFAULT".to_string(),
                    desc: "bad address".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff1",
                Error {
                    name: "ENOTBLK".to_string(),
                    desc: "block device required".to_string(),
                },
            );
            tmp.insert(
                "0xfffffffffffffff0",
                Error {
                    name: "EBUSY".to_string(),
                    desc: "device or resource busy".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffef",
                Error {
                    name: "EEXIST".to_string(),
                    desc: "file exists".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffee",
                Error {
                    name: "EXDEV".to_string(),
                    desc: "invalid cross-device link".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffed",
                Error {
                    name: "ENODEV".to_string(),
                    desc: "no such device".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffec",
                Error {
                    name: "ENOTDIR".to_string(),
                    desc: "not a directory".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffeb",
                Error {
                    name: "EISDIR".to_string(),
                    desc: "is a directory".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffea",
                Error {
                    name: "EINVAL".to_string(),
                    desc: "invalid argument".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe9",
                Error {
                    name: "ENFILE".to_string(),
                    desc: "too many open files in system".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe8",
                Error {
                    name: "EMFILE".to_string(),
                    desc: "too many open files".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe7",
                Error {
                    name: "ENOTTY".to_string(),
                    desc: "inappropriate ioctl for device".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe6",
                Error {
                    name: "ETXTBSY".to_string(),
                    desc: "text file busy".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe5",
                Error {
                    name: "EFBIG".to_string(),
                    desc: "file too large".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe4",
                Error {
                    name: "ENOSPC".to_string(),
                    desc: "no space left on device".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe3",
                Error {
                    name: "ESPIPE".to_string(),
                    desc: "illegal seek".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe2",
                Error {
                    name: "EROFS".to_string(),
                    desc: "read-only file system".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe1",
                Error {
                    name: "EMLINK".to_string(),
                    desc: "too many links".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffe0",
                Error {
                    name: "EPIPE".to_string(),
                    desc: "broken pipe".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffdf",
                Error {
                    name: "EDOM".to_string(),
                    desc: "numerical argument out of domain".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffde",
                Error {
                    name: "ERANGE".to_string(),
                    desc: "numerical result out of range".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffdd",
                Error {
                    name: "EDEADLK".to_string(),
                    desc: "resource deadlock avoided".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffdc",
                Error {
                    name: "ENAMETOOLONG".to_string(),
                    desc: "file name too long".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffdb",
                Error {
                    name: "ENOLCK".to_string(),
                    desc: "no locks available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffda",
                Error {
                    name: "ENOSYS".to_string(),
                    desc: "function not implemented".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd9",
                Error {
                    name: "ENOTEMPTY".to_string(),
                    desc: "directory not empty".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd8",
                Error {
                    name: "ELOOP".to_string(),
                    desc: "too many levels of symbolic links".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd6",
                Error {
                    name: "ENOMSG".to_string(),
                    desc: "no message of desired type".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd5",
                Error {
                    name: "EIDRM".to_string(),
                    desc: "identifier removed".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd4",
                Error {
                    name: "ECHRNG".to_string(),
                    desc: "channel number out of range".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd3",
                Error {
                    name: "EL2NSYNC".to_string(),
                    desc: "level 2 not synchronized".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd2",
                Error {
                    name: "EL3HLT".to_string(),
                    desc: "level 3 halted".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd1",
                Error {
                    name: "EL3RST".to_string(),
                    desc: "level 3 reset".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffd0",
                Error {
                    name: "ELNRNG".to_string(),
                    desc: "link number out of range".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffcf",
                Error {
                    name: "EUNATCH".to_string(),
                    desc: "protocol driver not attached".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffce",
                Error {
                    name: "ENOCSI".to_string(),
                    desc: "no CSI structure available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffcd",
                Error {
                    name: "EL2HLT".to_string(),
                    desc: "level 2 halted".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffcc",
                Error {
                    name: "EBADE".to_string(),
                    desc: "invalid exchange".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffcb",
                Error {
                    name: "EBADR".to_string(),
                    desc: "invalid request descriptor".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffca",
                Error {
                    name: "EXFULL".to_string(),
                    desc: "exchange full".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc9",
                Error {
                    name: "ENOANO".to_string(),
                    desc: "no anode".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc8",
                Error {
                    name: "EBADRQC".to_string(),
                    desc: "invalid request code".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc7",
                Error {
                    name: "EBADSLT".to_string(),
                    desc: "invalid slot".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc5",
                Error {
                    name: "EBFONT".to_string(),
                    desc: "bad font file format".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc4",
                Error {
                    name: "ENOSTR".to_string(),
                    desc: "device not a stream".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc3",
                Error {
                    name: "ENODATA".to_string(),
                    desc: "no data available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc2",
                Error {
                    name: "ETIME".to_string(),
                    desc: "timer expired".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc1",
                Error {
                    name: "ENOSR".to_string(),
                    desc: "out of streams resources".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffc0",
                Error {
                    name: "ENONET".to_string(),
                    desc: "machine is not on the network".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffbf",
                Error {
                    name: "ENOPKG".to_string(),
                    desc: "package not installed".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffbe",
                Error {
                    name: "EREMOTE".to_string(),
                    desc: "object is remote".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffbd",
                Error {
                    name: "ENOLINK".to_string(),
                    desc: "link has been severed".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffbc",
                Error {
                    name: "EADV".to_string(),
                    desc: "advertise error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffbb",
                Error {
                    name: "ESRMNT".to_string(),
                    desc: "srmount error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffba",
                Error {
                    name: "ECOMM".to_string(),
                    desc: "communication error on send".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb9",
                Error {
                    name: "EPROTO".to_string(),
                    desc: "protocol error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb8",
                Error {
                    name: "EMULTIHOP".to_string(),
                    desc: "multihop attempted".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb7",
                Error {
                    name: "EDOTDOT".to_string(),
                    desc: "RFS specific error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb6",
                Error {
                    name: "EBADMSG".to_string(),
                    desc: "bad message".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb5",
                Error {
                    name: "EOVERFLOW".to_string(),
                    desc: "value too large for defined data type".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb4",
                Error {
                    name: "ENOTUNIQ".to_string(),
                    desc: "name not unique on network".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb3",
                Error {
                    name: "EBADFD".to_string(),
                    desc: "file descriptor in bad state".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb2",
                Error {
                    name: "EREMCHG".to_string(),
                    desc: "remote address changed".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb1",
                Error {
                    name: "ELIBACC".to_string(),
                    desc: "can not access a needed shared library".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffb0",
                Error {
                    name: "ELIBBAD".to_string(),
                    desc: "accessing a corrupted shared library".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffaf",
                Error {
                    name: "ELIBSCN".to_string(),
                    desc: ".lib section in a.out corrupted".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffae",
                Error {
                    name: "ELIBMAX".to_string(),
                    desc: "attempting to link in too many shared libraries".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffad",
                Error {
                    name: "ELIBEXEC".to_string(),
                    desc: "cannot exec a shared library directly".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffac",
                Error {
                    name: "EILSEQ".to_string(),
                    desc: "invalid or incomplete multibyte or wide character".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffab",
                Error {
                    name: "ERESTART".to_string(),
                    desc: "interrupted system call should be restarted".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffaa",
                Error {
                    name: "ESTRPIPE".to_string(),
                    desc: "streams pipe error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa9",
                Error {
                    name: "EUSERS".to_string(),
                    desc: "too many users".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa8",
                Error {
                    name: "ENOTSOCK".to_string(),
                    desc: "socket operation on non-socket".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa7",
                Error {
                    name: "EDESTADDRREQ".to_string(),
                    desc: "destination address required".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa6",
                Error {
                    name: "EMSGSIZE".to_string(),
                    desc: "message too long".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa5",
                Error {
                    name: "EPROTOTYPE".to_string(),
                    desc: "protocol wrong type for socket".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa4",
                Error {
                    name: "ENOPROTOOPT".to_string(),
                    desc: "protocol not available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa3",
                Error {
                    name: "EPROTONOSUPPORT".to_string(),
                    desc: "protocol not supported".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa2",
                Error {
                    name: "ESOCKTNOSUPPORT".to_string(),
                    desc: "socket type not supported".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa1",
                Error {
                    name: "ENOTSUP".to_string(),
                    desc: "operation not supported".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffffa0",
                Error {
                    name: "EPFNOSUPPORT".to_string(),
                    desc: "protocol family not supported".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9f",
                Error {
                    name: "EAFNOSUPPORT".to_string(),
                    desc: "address family not supported by protocol".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9e",
                Error {
                    name: "EADDRINUSE".to_string(),
                    desc: "address already in use".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9d",
                Error {
                    name: "EADDRNOTAVAIL".to_string(),
                    desc: "cannot assign requested address".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9c",
                Error {
                    name: "ENETDOWN".to_string(),
                    desc: "network is down".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9b",
                Error {
                    name: "ENETUNREACH".to_string(),
                    desc: "network is unreachable".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff9a",
                Error {
                    name: "ENETRESET".to_string(),
                    desc: "network dropped connection on reset".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff99",
                Error {
                    name: "ECONNABORTED".to_string(),
                    desc: "software caused connection abort".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff98",
                Error {
                    name: "ECONNRESET".to_string(),
                    desc: "connection reset by peer".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff97",
                Error {
                    name: "ENOBUFS".to_string(),
                    desc: "no buffer space available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff96",
                Error {
                    name: "EISCONN".to_string(),
                    desc: "transport endpoint is already connected".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff95",
                Error {
                    name: "ENOTCONN".to_string(),
                    desc: "transport endpoint is not connected".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff94",
                Error {
                    name: "ESHUTDOWN".to_string(),
                    desc: "cannot send after transport endpoint shutdown".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff93",
                Error {
                    name: "ETOOMANYREFS".to_string(),
                    desc: "too many references: cannot splice".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff92",
                Error {
                    name: "ETIMEDOUT".to_string(),
                    desc: "connection timed out".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff91",
                Error {
                    name: "ECONNREFUSED".to_string(),
                    desc: "connection refused".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff90",
                Error {
                    name: "EHOSTDOWN".to_string(),
                    desc: "host is down".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8f",
                Error {
                    name: "EHOSTUNREACH".to_string(),
                    desc: "no route to host".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8e",
                Error {
                    name: "EALREADY".to_string(),
                    desc: "operation already in progress".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8d",
                Error {
                    name: "EINPROGRESS".to_string(),
                    desc: "operation now in progress".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8c",
                Error {
                    name: "ESTALE".to_string(),
                    desc: "stale file handle".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8b",
                Error {
                    name: "EUCLEAN".to_string(),
                    desc: "structure needs cleaning".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff8a",
                Error {
                    name: "ENOTNAM".to_string(),
                    desc: "not a XENIX named type file".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff89",
                Error {
                    name: "ENAVAIL".to_string(),
                    desc: "no XENIX semaphores available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff88",
                Error {
                    name: "EISNAM".to_string(),
                    desc: "is a named type file".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff87",
                Error {
                    name: "EREMOTEIO".to_string(),
                    desc: "remote I/O error".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff86",
                Error {
                    name: "EDQUOT".to_string(),
                    desc: "disk quota exceeded".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff85",
                Error {
                    name: "ENOMEDIUM".to_string(),
                    desc: "no medium found".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff84",
                Error {
                    name: "EMEDIUMTYPE".to_string(),
                    desc: "wrong medium type".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff83",
                Error {
                    name: "ECANCELED".to_string(),
                    desc: "operation canceled".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff82",
                Error {
                    name: "ENOKEY".to_string(),
                    desc: "required key not available".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff81",
                Error {
                    name: "EKEYEXPIRED".to_string(),
                    desc: "key has expired".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff80",
                Error {
                    name: "EKEYREVOKED".to_string(),
                    desc: "key has been revoked".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff7f",
                Error {
                    name: "EKEYREJECTED".to_string(),
                    desc: "key was rejected by service".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff7e",
                Error {
                    name: "EOWNERDEAD".to_string(),
                    desc: "owner died".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff7d",
                Error {
                    name: "ENOTRECOVERABLE".to_string(),
                    desc: "state not recoverable".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff7c",
                Error {
                    name: "ERFKILL".to_string(),
                    desc: "operation not possible due to RF-kill".to_string(),
                },
            );
            tmp.insert(
                "0xffffffffffffff7b",
                Error {
                    name: "EHWPOISON".to_string(),
                    desc: "memory page has hardware error".to_string(),
                },
            );
            tmp
        }
    }};
}
