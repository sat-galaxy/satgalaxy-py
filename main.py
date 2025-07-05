from satgalaxy import solver
def main():
    solve= solver.minisat.MinisatSolver()
    solve.add_clause([1,2,3])
    if solve.solve() :
        print("Model ",solve.model())
    print("Hello from satgalaxy-py!")


if __name__ == "__main__":
    main()
