import pandas as pd
import sys
from copy_test_case_from_excel.buildings import write_test_case_buildings, write_excel_comparison_buildings
from copy_test_case_from_excel.energy import write_test_case_energy, write_excel_comparison_energy
from copy_test_case_from_excel.mobility import write_test_case_mobility, write_excel_comparison_mobility


if __name__ == "__main__":
    inputs = pd.read_excel(sys.argv[1], sheet_name="Eingabe Ist")
    results = pd.read_excel(sys.argv[1], sheet_name="Rechenwerk")
    measures = pd.read_excel(sys.argv[1], sheet_name="Ergebnisse")

    #buildings
    write_test_case_buildings(inputs, measures)
    write_excel_comparison_buildings(results)

    #energy
    # write_test_case_energy(inputs, measures)
    # write_excel_comparison_energy(results)

    #mobility
    # write_test_case_mobility(inputs, measures)
    # write_excel_comparison_mobility(results)



