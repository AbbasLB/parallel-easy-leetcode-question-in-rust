import subprocess
import matplotlib.pyplot as plt
import os
def run_rust_program(problem_size, threads):
    rust_project_path = os.path.join(os.path.dirname(__file__), '..\\')
    cargo_path_with_quotes = f'"{rust_project_path}Cargo.toml"' 
    command = f'cargo run --manifest-path {cargo_path_with_quotes} --release {problem_size} {threads}'
    result = subprocess.run(command, shell=True, capture_output=True, text=True, cwd=rust_project_path)
    output = result.stdout.split('\n')
    sequential_time = float(output[0])
    parallel_time = float(output[1])
    return sequential_time, parallel_time




def speedup(sequential_time, parallel_time):
    return sequential_time / parallel_time

def main():
    problem_size = 100000000  # Adjust this value as needed
    threads = list(range(1, 17))
    sequential_times = []
    parallel_times = []
    speedups = []

    for t in threads:
        sequential_time, parallel_time = run_rust_program(problem_size, t)
        sequential_times.append(sequential_time)
        parallel_times.append(parallel_time)
        speedups.append(speedup(sequential_time, parallel_time))
        print('thread t=',str(t),' sequential= ',str(sequential_time),' parallel= ',str(parallel_time),' speedup= '+str(speedup(sequential_time, parallel_time)))

    plt.plot(threads, speedups, marker='o')
    plt.xlabel('Number of Threads')
    plt.ylabel('Speedup')
    plt.title(f'Speedup vs. Number of Threads for Problem Size {problem_size}')
    plt.savefig('speedup.pdf', format='pdf', dpi=1200)
    plt.show()

if __name__ == '__main__':
    main()
