#!/usr/bin/env python3

# Requirements
# Install psutil and py-cpuinfo

import matplotlib.pyplot as plt
import csv
import math
import psutil
import platform
import cpuinfo
import argparse


def parse_args():
    ap = argparse.ArgumentParser(
        description='Draws charts comparing benchmark results from basic_threads')

    ap.add_argument('-i', '--input', required=True,
                    help='CSV file containing benchmarch results from test.py.')
    ap.add_argument('-t', '--type', required=True,
                    help='Set to "split" or to "max": which of the test types in the results file to chart.')
    ap.add_argument(
        '--scaling', action='store_true',
        help="Chart how many times faster the computation is vs. the number of threads.  Only works for split test type.")

    ap_args = vars(ap.parse_args())

    if ap_args['scaling'] and ap_args['type'] != 'split':
        ap.error("The scaling flag can only be used with type split")

    return {
        'input': ap_args['input'],
        'type': ap_args['type'],
        'scaling': ap_args['scaling'],
    }


def get_size(bytes, suffix="B"):
    """
    Scale bytes to its proper format
    e.g:
        1253656 => '1.20MB'
        1253656678 => '1.17GB'
    """
    factor = 1024
    for unit in ["", "K", "M", "G", "T", "P"]:
        if bytes < factor:
            return f"{bytes:.2f}{unit}{suffix}"
        bytes /= factor


def get_sys_info():
    phys_cores = psutil.cpu_count(logical=False)
    log_cores = psutil.cpu_count(logical=True)
    cpu_freq = psutil.cpu_freq()
    cpu_freq = cpu_freq.max
    vmem = psutil.virtual_memory()
    total_mem = get_size(vmem.total)
    return {
        'cpu': {
            'name': cpuinfo.get_cpu_info()['brand_raw'],
            'cores': {
                'logical': log_cores,
                'physical': phys_cores,
            },
            'freq': cpu_freq
        },
        'total_memory': total_mem,
    }


def read_results(file, ty):
    results = {}
    with open(file) as csv_file:
        csv_reader = csv.DictReader(csv_file, delimiter=',')
        for row in csv_reader:
            if row['Type'] != ty:
                continue

            if int(row['Test']) not in results:
                results[int(row['Test'])] = {}
            results[int(row['Test'])][int(row['Th'])] = {
                'avg': float(row['Avg']),
                'min': float(row['Min']),
                'max': float(row['Max']),
            }

    return results


def extract_ts(results, loops, stat):
    g = results[loops]
    line = [[th, g[th][stat]] for th in g]
    line.sort()
    x = [a[0] for a in line]
    y = [a[1] for a in line]
    return x, y


def compute_ideal_time(results, loops):
    d = results[loops]
    one_th = d[1]['avg']
    x = []
    y = []
    for th in d:
        x.append(th)
        y.append(one_th/th)
    return x, y


def compute_scaling(results, loops):
    d = results[loops]
    one_th = d[1]['avg']
    x = []
    y = []
    for th in d:
        scaling = one_th / d[th]['avg']

        x.append(th)
        y.append(scaling)
    return x, y


def compute_perfect_scaling(results, loops):
    d = results[loops]
    x = []
    y = []
    for th in d:
        x.append(th)
        y.append(th)
    return x, y


# parse CLI arguments
args = parse_args()
print(args)

# Load CSV test results
source_csv = args['input']
results = read_results(source_csv, args['type'])

# Construct Chart Title
sys_info = get_sys_info()
plt.suptitle('(Time For 1 Thread)/(Time For N Threads) on CPU:{} Mem: {}'.format(
    sys_info['cpu']['name'], sys_info['total_memory']))

# Configure the chart layout to have at most 3 columns
cols = min(3, len(results))
rows = math.ceil(len(results) / cols + min(1, len(results) % cols))

# Create Chart
subplot = 1
for loops in results:
    plt.subplot(rows, cols, subplot)
    plt.title('Loops {}'.format(loops))
    subplot += 1

    if args['scaling']:
        # compute actual scaling
        x, y = compute_scaling(results, loops)
        plt.plot(x, y, '-o', label='Actual')
        plt.xlabel('Threads')
        plt.ylabel('Times Faster')

        # compute perfect scaling
        x, y = compute_perfect_scaling(results, loops)
        plt.plot(x, y, '--', label='Perfect')
        plt.xlabel('Threads')
        plt.ylabel('Times Faster')
    else:
        x, y = extract_ts(results, loops, 'min')
        plt.plot(x, y, '--', label='Min')
        plt.xlabel('Threads')
        plt.ylabel('Seconds')

        x, y = extract_ts(results, loops, 'max')
        plt.plot(x, y, '--', label='Max')
        plt.xlabel('Threads')
        plt.ylabel('Seconds')

        x, y = extract_ts(results, loops, 'avg')
        plt.plot(x, y, label='Avg')
        plt.xlabel('Threads')
        plt.ylabel('Seconds')

    plt.legend()

plt.show()
