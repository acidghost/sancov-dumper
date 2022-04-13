#!/usr/bin/env python3
import os, shutil, sys

def main(args: list[str]):
    has_c = False
    for arg in args:
        if arg == '-c':
            has_c = True
            break
    new_args = ['clang']
    if has_c:
        new_args.append('-fsanitize-coverage=trace-pc-guard')
    new_args += args
    if not has_c:
        new_args.append(os.environ['SANCOV_RT_LIB'])
    if clang := shutil.which('clang'):
        os.execv(clang, new_args)
    raise Exception('could not find clang')

if __name__ == '__main__':
    main(sys.argv[1:])
