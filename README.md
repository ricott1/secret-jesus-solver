# Secret Jesus Solver

The **Secret Jesus Solver** is a simulation and SAT solver for [Secret Jesus](https://secretjes.us). The game is simulated randomly and the solver tries to identify Jesus based on the played prodigies.

## How to Run

1. **Install Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd secret-jesus-solver
3. **Run the solver**:
```bash
cargo run --release -- [OPTIONS]
```
-   `-p, --players <PLAYERS>`  The number of players to simulate in the game. [default: 7].
- `-s, --seed <SEED>`: Optional random seed for reproducibility. [default: 0].
- `-v, --verbose`: Enable verbose output for detailed game simulation.

Example:
```bash
cargo run --release -- --players 6 --ssed 15 --verbose
```