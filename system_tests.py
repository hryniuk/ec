#!/usr/bin/env python
from subprocess import call

import argparse
import glob
import os
import subprocess
import sys


def read_out(filepath):
    if filepath == None:
        return ""

    with open(filepath, 'r') as f:
        return f.read()


def assert_eq_output(expected, output):
    return (expected is None and output is None) or expected == output


def run_test(testcase, binary_path, alf_filepath, out_filepath):
    out = subprocess.check_output([binary_path, alf_filepath],
            universal_newlines=True)
    if assert_eq_output(read_out(out_filepath), out):
        print("{} ok".format(testcase))
        return 0
    else:
        print("{} fail".format(testcase))
        return 1


def testcase_name(filepath):
    return os.path.splitext(os.path.basename(filepath))[0]


def generate_tests(alfs, outs):
    alf_by_testcase = {testcase_name(path): path for path in alfs}
    out_by_testcase = {testcase_name(path): path for path in outs}

    for testcase, alf in alf_by_testcase.items():
        yield testcase, alf, out_by_testcase.get(testcase)


def main(binary_path, test_files_dir):
    alf_dir, out_dir = map(assert_exists,
                           map(lambda x: os.path.join(test_files_dir, x),
                               ('alf', 'out')))

    failed = 0

    for testcase, alf, out in generate_tests(
            glob.glob(os.path.join(alf_dir, '*.alf')),
            glob.glob(os.path.join(out_dir, '*.out'))):
        if out is None:
            print("error: missing out file for {testcase}")
            continue

        failed += run_test(testcase, binary_path, alf, out)

    if failed > 0:
        sys.exit(1)
    sys.exit(0)


def read_paths_from_command_line():
    parser = argparse.ArgumentParser(
        description='Run integration tests with given binary and test files')
    parser.add_argument('--bin',
                        help='ec binary path',
                        required=True)
    parser.add_argument('--test-files',
                        help='path to test_files directory; it'
                             'has to contain alf and out directories')

    args = parser.parse_args()

    return map(assert_exists, (args.bin, args.test_files))


def assert_exists(path):
    if not os.path.exists(path):
        print("{} doesn't exist".format(path))
        sys.exit(1)
    return path


if __name__ == '__main__':
    bin_dir, test_files_dir = read_paths_from_command_line()
    main(bin_dir, test_files_dir)
