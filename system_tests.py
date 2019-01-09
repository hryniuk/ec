#!/usr/bin/env python
from subprocess import call

import argparse
import glob
import os
import subprocess
import sys


def read(filepath):
    if filepath == None:
        return ""

    with open(filepath, 'r') as f:
        return f.read()


def assert_eq_output(expected, output):
    return (expected is None and output is None) or expected.strip() == output.strip()


def run_test(testcase, binary_path, alf_filepath, in_filepath, out_filepath):
    input = read(in_filepath)
    out = subprocess.check_output([binary_path, '-f', alf_filepath, '-q'],
                                  input=input,
                                  universal_newlines=True)
    expected_output = read(out_filepath)
    if assert_eq_output(expected_output, out):
        print("{} ok".format(testcase))
        return 0
    else:
        print("{} fail".format(testcase))
        print("### ec output:\n{}".format(out))
        print("### expected output:\n{}".format(expected_output))
        return 1


def testcase_name(filepath):
    return os.path.splitext(os.path.basename(filepath))[0]


def generate_tests(alfs, ins, outs):
    alf_by_testcase = {testcase_name(path): path for path in alfs}
    in_by_testcase = {testcase_name(path): path for path in ins}
    out_by_testcase = {testcase_name(path): path for path in outs}

    for testcase, alf in alf_by_testcase.items():
        yield testcase, alf, in_by_testcase.get(testcase), out_by_testcase.get(testcase)


def main(binary_path, test_files_dir):
    alf_dir, in_dir, out_dir = map(assert_exists,
                           map(lambda x: os.path.join(test_files_dir, x),
                               ('alf', 'in', 'out')))

    failed = 0

    for testcase, alf, in_, out in generate_tests(
            glob.glob(os.path.join(alf_dir, '*.alf')),
            glob.glob(os.path.join(in_dir, '*.in')),
            glob.glob(os.path.join(out_dir, '*.out'))):
        if out is None:
            print("error: missing out file for {}".format(testcase))
            continue

        failed += run_test(testcase, binary_path, alf, in_, out)

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
